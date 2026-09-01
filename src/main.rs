use clap::Parser;
use config::Config;
use git2::{Commit, Oid, Repository};
use mailmap::{Author, Mailmap};
use regex::{Regex, RegexBuilder};
use reviewers::Reviewers;
use semver::Version;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{BufWriter, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{cmp, fmt, str};

mod config;
mod error;
mod reviewers;
mod score;
mod site;

use crate::score::{AuthorScore, author_map_to_scores};
use error::ErrorContext;

/// Convert a commit signature to an `Author`.
///
/// Since `Author` is defined in the mailmap crate, this trait is needed to
/// allow adding an extra method to the `Author` type.
trait ToAuthor {
    /// Convert a git commit signature to an `Author`.
    fn from_sig(sig: git2::Signature<'_>) -> Author;
}

impl ToAuthor for Author {
    fn from_sig(sig: git2::Signature<'_>) -> Author {
        let name = sig.name().unwrap_or_else(|| panic!("no name for {}", sig));
        let email = sig
            .email()
            .unwrap_or_else(|| panic!("no email for {}", sig));

        Author::new(name.to_string(), email.to_string())
    }
}

/// Map authors to their commits.
#[derive(Clone)]
pub struct AuthorMap {
    /// Mapping of each Author to the commits they authored or co-authored.
    map: HashMap<Author, HashSet<Oid>>,
}

impl AuthorMap {
    /// Create an empty `AuthorMap`.
    fn new() -> Self {
        AuthorMap {
            map: HashMap::new(),
        }
    }

    /// Add a commit authored or co-authored by the given `Author`.
    ///
    /// If the author is not already included in the map, they are added.
    fn add(&mut self, author: Author, commit: Oid) {
        self.map.entry(author).or_default().insert(commit);
    }

    /// Iterate over each author and the number of commits that they (co-)authored.
    fn iter(&self) -> impl Iterator<Item = (&Author, usize)> {
        self.map.iter().map(|(k, v)| (k, v.len()))
    }

    /// Merge in the authorship data from another instance.
    fn extend(&mut self, other: Self) {
        for (author, set) in other.map {
            self.map.entry(author).or_default().extend(set);
        }
    }
}

pub struct AuthorsWithScores {
    pub authors: AuthorMap,
    pub scores: Vec<AuthorScore>,
}

impl AuthorsWithScores {
    fn new(authors: AuthorMap) -> Self {
        let scores = author_map_to_scores(&authors);
        Self { authors, scores }
    }
}

/// Run a `git` command with the given arguments.
///
/// # Panics
///
/// Panics if the `git` command cannot be spawned
fn git(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("git");
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    let out = cmd.spawn();
    let mut out = match out {
        Ok(v) => v,
        Err(err) => {
            panic!("Failed to spawn command `{:?}`: {:?}", cmd, err);
        }
    };

    let status = out.wait().expect("waited");

    if !status.success() {
        eprintln!("failed to run `git {:?}`: {:?}", args, status);
        return Err(std::io::Error::from(std::io::ErrorKind::Other).into());
    }

    let mut stdout = Vec::new();
    out.stdout.unwrap().read_to_end(&mut stdout).unwrap();
    Ok(String::from_utf8_lossy(&stdout).into_owned())
}

lazy_static::lazy_static! {
    static ref UPDATED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Create or update the bare clone of the git repo at the given URL
///
/// If a clone of the repo already exists, it is only updated if
/// [`should_update()`]  returns true.
///
/// On success, the returned Result contains a PathBuf with the path to the
/// clone.
fn update_repo(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut slug = url;
    let prefix = "https://github.com/";
    if slug.starts_with(prefix) {
        slug = &slug[prefix.len()..];
    }
    let prefix = "git://github.com/";
    if slug.starts_with(prefix) {
        slug = &slug[prefix.len()..];
    }
    let prefix = "https://git.chromium.org/";
    if slug.starts_with(prefix) {
        slug = &slug[prefix.len()..];
    }
    let suffix = ".git";
    if slug.ends_with(suffix) {
        slug = &slug[..slug.len() - suffix.len()];
    }

    let path_s = format!("repos/{}", slug);
    let path = PathBuf::from(&path_s);
    if !UPDATED.lock().unwrap().insert(slug.to_string()) {
        return Ok(path);
    }
    if path.exists() {
        if should_update() {
            // we know for sure the path_s does *not* contain .git as we strip it, so this is a safe
            // temp directory
            let tmp = format!("{}.git", path_s);
            std::fs::rename(&path, &tmp)?;
            git(&[
                "clone",
                "--bare",
                "--dissociate",
                "--reference",
                &tmp,
                url,
                &path_s,
            ])?;
            std::fs::remove_dir_all(&tmp)?;
        }
    } else {
        git(&["clone", "--bare", url, &path_s])?;
    }
    Ok(path)
}

/// Determine if existing git clones should be updated.
///
/// Clones that already exist are only updated if the first command line
/// argument specified was `--refresh`.
fn should_update() -> bool {
    std::env::var("REFRESH").is_ok()
}

/// Information about a git tag or other reference to treat as a tag.
#[derive(Clone)]
pub struct VersionTag {
    /// Some custom name, e.g. "Rust 1.94.0" or "Beta".
    name: String,
    /// The parsed Version for this tag.
    version: Version,
    /// The raw name of the tag or commit.
    raw_tag: String,
    /// The commit for this tag or reference.
    commit: Oid,
    /// Whether this version is still being developed.
    ///
    /// This should only be true for the "Beta" and "Nightly" versions.
    in_progress: bool,
}

impl fmt::Display for VersionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl std::hash::Hash for VersionTag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

impl cmp::Eq for VersionTag {}
impl cmp::PartialEq for VersionTag {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl cmp::PartialOrd for VersionTag {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for VersionTag {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl fmt::Debug for VersionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
    }
}

struct VersionCommits {
    main: Vec<Oid>,
    submodules: Vec<(Submodule, Vec<Oid>)>,
}

impl VersionCommits {
    fn total_commits(&self) -> u64 {
        self.main.len() as u64
            + self
                .submodules
                .iter()
                .map(|(_, commits)| commits.len() as u64)
                .sum::<u64>()
    }
}

/// Identify the versions that have been tagged in the given repo.
///
/// A [`VersionTag`] is created for each tagged commit in the given repository
/// where either
/// * the name of the tag can be parsed with [`Version::parse()`]
/// * the name of the tag, followed by ".0", can be parsed with
///   [`Version::parse()`]
///
/// The values of [`VersionTag::version`] are the results of the successful
/// [`Version::parse()`] calls (i.e. they might include extra ".0"s not in the
/// tag names). Each of the returned version tags has the
/// [`in_progress`][VersionTag::in_progress] field as `false`.
fn get_versions(repo: &Repository) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
    let tags = repo
        .tag_names(None)?
        .into_iter()
        .flatten()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    let mut versions = tags
        .iter()
        .filter_map(|tag| {
            Version::parse(tag)
                .or_else(|_| Version::parse(&format!("{}.0", tag)))
                .ok()
                .map(|v| VersionTag {
                    name: format!("Rust {}", v),
                    version: v,
                    raw_tag: tag.clone(),
                    commit: repo
                        .revparse_single(tag)
                        .unwrap()
                        .peel_to_commit()
                        .unwrap()
                        .id(),
                    in_progress: false,
                })
        })
        .collect::<Vec<_>>();
    versions.sort();
    Ok(versions)
}

/// Identify the co-authors, if any, of a commit
///
/// Co-authors are determined based on the commit message having lines starting
/// `Co-authored-by: ` followed by a name and then an email enclosed in `<>`.
fn commit_coauthors(commit: &Commit) -> Vec<Author> {
    let mut coauthors = vec![];
    if let Some(msg) = commit.message_raw() {
        lazy_static::lazy_static! {
            static ref RE: Regex =
                RegexBuilder::new(r"^Co-authored-by: (?P<name>.*) <(?P<email>.*)>")
                    .case_insensitive(true)
                    .build()
                    .unwrap();
        }

        for line in msg.lines().rev() {
            if line.starts_with("Co-authored-by")
                && let Some(caps) = RE.captures(line)
            {
                coauthors.push(Author::new(
                    caps["name"].to_string(),
                    caps["email"].to_string(),
                ));
            }
        }
    }
    coauthors
}

/// Build up an [`AuthorMap`] of commits authored between `from` and `to`.
///
/// This function is a wrapper around [`build_author_map_`] to add additional
/// context to any errors; see that function for further documentation.
fn build_author_map(
    repo: &Repository,
    reviewers: &Reviewers,
    mailmap: &Mailmap,
    commits: &[Oid],
) -> Result<AuthorMap, Box<dyn std::error::Error>> {
    match build_author_map_(repo, reviewers, mailmap, commits) {
        Ok(o) => Ok(o),
        Err(err) => Err(ErrorContext(
            format!("build_author_map(repo={})", repo.path().display(),),
            err,
        ))?,
    }
}

/// Determine if a commit is a "rollup" merge commit
///
/// Rolloup commits are those whose commit messages start with "Rollup merge of #"
fn is_rollup_commit(commit: &Commit) -> bool {
    let summary = commit.summary().unwrap_or("");
    summary.starts_with("Rollup merge of #")
}

/// Parse a commit to identify which reviewer(s) should be created as the author
/// of a commit
///
/// For commits that were not authored by bors and are neither committed by
/// GitHub or considered rollup commits (see [`is_rollup_commit`]), no reviewers
/// are identified.
///
/// For non-merge commits, no reviewers are identified.
///
/// For commits where at least one line of the commit message contains ` r=`,
/// the reviewers are those listed after that ` r=`. If no such line exists,
/// for commits where at least one line of the commit message starts with
/// `Reviewed-by: `, the reviewers are those listed on that line. For commits
/// where neither type of line exists, the commit message must be
/// exactly "automated merge\n".
///
/// # Panics
///
/// For commits that are merge commits and are either authored by bors or
/// both committed by GitHub and considered rollup commits, we try to identify
/// reviewers. If no line of the commit message contains ` r=` or starts with
/// `Reviewed-by: `, the commit message must be exactly "automated merge\n",
/// otherwise panics.
fn parse_bors_reviewer(
    reviewers: &Reviewers,
    repo: &Repository,
    commit: &Commit,
) -> Result<Option<Vec<Author>>, ErrorContext> {
    let is_old_bors =
        commit.author().name_bytes() == b"bors" && commit.committer().name_bytes() == b"bors";
    // This username was used for merges for a ~week from January 7 to January 12 2026 on the
    // rust-lang/rust repository.
    let is_new_bors = commit.author().name_bytes() == b"rust-bors[bot]";
    let is_bors = is_old_bors || is_new_bors;

    if !is_bors && (commit.committer().name_bytes() != b"GitHub" || !is_rollup_commit(commit)) {
        return Ok(None);
    }

    // Skip non-merge commits
    if commit.parents().count() == 1 {
        return Ok(None);
    }

    let to_author = |list: &str| -> Result<Vec<Author>, ErrorContext> {
        list.trim_end_matches('.')
            .split([',', '+'])
            .map(|r| r.trim_start_matches('@'))
            .map(|r| r.trim_end_matches('`'))
            .map(|r| r.trim())
            .filter(|r| !r.is_empty())
            .filter(|r| *r != "<try>")
            .inspect(|r| {
                if !r.chars().all(|c| {
                    c.is_alphabetic() || c.is_ascii_digit() || c == '-' || c == '_' || c == '='
                }) {
                    eprintln!(
                        "warning: to_author for {} contained non-alphabetic characters: {:?}",
                        commit.id(),
                        r
                    );
                }
            })
            // Iterator is now of strings that are not empty, not `<try>`,
            // do not container `,` or `+`, do not start with `@`, do not end
            // with a '`', and do not start or end with whitespace
            .map(|r| {
                reviewers.to_author(r).map_err(|e| {
                    ErrorContext(
                        format!("reviewer: {:?}, commit: {}", r, commit.id()),
                        e.into(),
                    )
                })
            })
            // Iterator is now of `Result<Option<Author>, ErrorContext>` items
            .flat_map(|r| r.transpose())
            // Each r.transpose() call returned an Option<Result<Author, ErrorContext>>
            // for the `map` part of `flat_map()`; the `flat` part used
            // `Option::into_iter()` to ignore the `None` options and create an
            // iterator of `Result<Author, ErrorContext>` items.
            // Using the `FromIterator` impl for Result<V, E>, the `collect()`
            // call below will result in either 1) the first `ErrorContext` in
            // the iterator, or 2) all of the `Author`s in the iterator, if there
            // were no errors
            .collect::<Result<Vec<_>, ErrorContext>>()
    };

    let message = commit.message().unwrap_or("");
    let mut reviewers = if let Some(line) = message.lines().find(|l| l.contains(" r=")) {
        let start = line.find("r=").unwrap() + 2;
        let end = line[start..]
            .find(' ')
            .map(|pos| pos + start)
            .unwrap_or(line.len());
        to_author(&line[start..end])?
    } else if let Some(line) = message.lines().find(|l| l.starts_with("Reviewed-by: ")) {
        let line = &line["Reviewed-by: ".len()..];
        to_author(line)?
    } else {
        // old bors didn't include r=
        if message != "automated merge\n" {
            panic!(
                "expected reviewer for bors merge commit {} in {:?}, message: {:?}",
                commit.id(),
                repo.path(),
                message
            );
        }
        return Ok(None);
    };
    reviewers.sort();
    reviewers.dedup();
    Ok(Some(reviewers))
}

/// Build up an [`AuthorMap`] of commits authored between `from` and `to`.
///
/// If `from` is empty, commits authored up to and including `to` are processed.
/// If `from` is non-empty, commits starting *after* `from` are processed, up
/// to and including the `to` commit.
///
/// For each commit processed, authorship is added to the `AuthorMap` result
/// according to the following rules:
/// * If the commit is **not** a rollup commit (see [`is_rollup_commit`]), the
///   git author of the commit is credited as an author of the commit.
/// * For every commit, any reviewers from by [`parse_bors_reviewer`] are
///   credited as authors of the commit.
/// * For every commit, any co-authors identified by [`commit_coauthors`] are
///   credited as authors of the commit.
///
/// Authors in the resulting map are canonicalized using
/// [`Mailmap::canonicalize`].
///
/// For any reviewer not recognized in [`parse_bors_reviewer`] (i.e. resulting
/// in `Err<ErrorContext>` where the error is [`reviewers::UnknownReviewer`])
/// a warning is printed to the standard error; any other error from
/// [`parse_bors_reviewer`] or other methods results in returning an error.
fn build_author_map_(
    repo: &Repository,
    reviewers: &Reviewers,
    mailmap: &Mailmap,
    commits: &[Oid],
) -> Result<AuthorMap, Box<dyn std::error::Error>> {
    let mut author_map = AuthorMap::new();
    for oid in commits {
        let commit = repo.find_commit(*oid)?;

        let mut commit_authors = Vec::new();
        if !is_rollup_commit(&commit) {
            // We ignore the author of rollup-merge commits, and account for
            // that author once by counting the reviewer of all bors merges. For
            // rollups, we consider that this is the most relevant person, which
            // is usually the case.
            //
            // Otherwise, a single rollup with N PRs attributes N commits to the author of the
            // rollup, which isn't fair.
            commit_authors.push(Author::from_sig(commit.author()));
        }
        match parse_bors_reviewer(reviewers, repo, &commit) {
            Ok(Some(reviewers)) => commit_authors.extend(reviewers),
            Ok(None) => {}
            Err(ErrorContext(msg, e)) => {
                if e.is::<reviewers::UnknownReviewer>() {
                    eprintln!("Unknown reviewer: {}", ErrorContext(msg, e));
                } else {
                    return Err(ErrorContext(msg, e).into());
                }
            }
        }
        commit_authors.extend(commit_coauthors(&commit));
        for author in commit_authors {
            let author = mailmap.canonicalize(&author);
            author_map.add(author, *oid);
        }
    }
    Ok(author_map)
}

/// Construct a `Mailmap` based on the latest commit in the given repository.
///
/// Returns an error if the latest commit cannot be retrieved or if it does not
/// contain a `.mailmap` file to read.
fn mailmap_from_repo(repo: &git2::Repository) -> Result<Mailmap, Box<dyn std::error::Error>> {
    let tree = repo.revparse_single("HEAD")?.peel_to_commit()?.tree()?;
    let file = tree.get_name(".mailmap");
    let file = match file {
        None => {
            eprintln!("No mailmap found");
            return Mailmap::from_string("".to_string());
        }
        Some(f) => f,
    };
    let file = String::from_utf8(file.to_object(repo)?.peel_to_blob()?.content().into())?;
    Mailmap::from_string(file)
}

fn generate_thanks(
    mailmap_path: Option<PathBuf>,
) -> Result<BTreeMap<VersionTag, AuthorMap>, Box<dyn std::error::Error>> {
    let path = update_repo("https://github.com/rust-lang/rust.git")?;
    let repo = git2::Repository::open(&path)?;

    let mailmap = match mailmap_path {
        Some(mailmap) => {
            let mailmap = std::fs::read_to_string(&mailmap)
                .map_err(|e| format!("Cannot read mailmap from {mailmap:?}: {e:?}"))?;
            Mailmap::from_string(mailmap)?
        }
        None => mailmap_from_repo(&repo)?,
    };
    let reviewers = Reviewers::new()?;

    let mut versions = get_versions(&repo)?;
    let last_full_stable = versions
        .iter()
        .rfind(|v| v.raw_tag.ends_with(".0"))
        .unwrap()
        .version
        .clone();

    // The nightly branch is the default one, fall back to "main" if it cannot
    // be read
    let nightly_branch = match repo.head() {
        Ok(reference) => match reference.shorthand() {
            Some(name) => name.to_string(),
            None => "main".to_string(),
        },
        Err(_) => "main".to_string(),
    };

    versions.push(VersionTag {
        name: String::from("Beta"),
        version: {
            let mut last = last_full_stable.clone();
            last.minor += 1;
            last
        },
        raw_tag: String::from("beta"),
        commit: repo
            .revparse_single("beta")
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id(),
        in_progress: true,
    });
    versions.push(VersionTag {
        name: String::from("Nightly"),
        version: {
            // main is plus 1 minor versions off of beta, which we just pushed
            let mut last = last_full_stable.clone();
            last.minor += 2;
            last
        },
        raw_tag: nightly_branch,
        commit: repo
            .revparse_single("HEAD")
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id(),
        in_progress: true,
    });

    let start = Instant::now();

    let by_version = gather_all_commits(&repo, versions)?;

    let version_count = by_version.len();
    let commit_count = by_version.values().map(|v| v.total_commits()).sum::<u64>();
    eprintln!(
        "Gathered {version_count} versions with {commit_count} total commits in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    let start = Instant::now();
    let version_map = by_version
        .into_iter()
        .map(|(version, data)| {
            let mut author_map = build_author_map(&repo, &reviewers, &mailmap, &data.main)?;
            for (submodule, commits) in data.submodules {
                let path = update_repo(&submodule.repository)?;
                let subrepo = Repository::open(path)?;
                author_map.extend(build_author_map(&subrepo, &reviewers, &mailmap, &commits)?);
            }
            Ok::<_, Box<dyn std::error::Error>>((version, author_map))
        })
        .collect::<Result<_, _>>()?;
    eprintln!(
        "Analyzed contributions in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(version_map)
}

/// Gather all commits for the given versions from the given repository, including all of its
/// submodules.
/// The commits are grouped by the individual versions.
fn gather_all_commits(
    repo: &Repository,
    versions: Vec<VersionTag>,
) -> Result<HashMap<VersionTag, VersionCommits>, Box<dyn std::error::Error>> {
    // Set of all commits that we visited
    let mut seen_commits = HashSet::new();

    let mut submodule_last_oid = HashMap::new();
    let mut last_version_oid: Option<Oid> = None;

    let mut by_version: HashMap<VersionTag, VersionCommits> = HashMap::new();

    // Re-opening the same repository multiple times causes us to unpack its object database
    // repeatedly. If we cache the repositories, this doesn't have to happen.
    let mut subrepo_cache: HashMap<String, Repository> = HashMap::new();

    let start = Instant::now();
    checkout_all_submodules(repo, &versions)?;
    eprintln!(
        "Checked out submodules in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    // Iterate all version from the oldest to the newest
    for version in &versions {
        let mut walk = repo.revwalk()?;

        // If we have a previous version, iterate from its commit to this commit
        // Note that stable version tag commits are usually "forked" off the commit mainline
        // Revwalk should take that into account
        if let Some(last) = last_version_oid {
            // Note: the left side of this range is exclusive, but that is what we want, because we
            // already visited the `last` commit in the previous version
            walk.push_range(&format!("{last}..{}", version.commit))?;
        } else {
            // If there is no previous version, iterate from this version to the start of the
            // commit history.
            walk.push(version.commit)?;
        }

        last_version_oid = Some(version.commit);

        // All commits of this version (that are not contained in other versions) from the main
        // repo. This is kept in a Vec for deterministic order.
        let mut version_commits = vec![];
        for commit in walk {
            let commit = commit?;
            version_commits.push(commit);
        }
        let mut submodule_commits = vec![];

        // Now find all submodules present in the repo at this commit
        let commit = repo.find_commit(version.commit)?;
        let modules = get_submodules(repo, &commit)?;
        for submodule in modules {
            let path = update_repo(&submodule.repository)?;

            let subrepo = subrepo_cache.get(&submodule.repository);
            let subrepo = match subrepo {
                Some(subrepo) => subrepo,
                None => {
                    // We do not use the entry API here because `open` returns a Result
                    subrepo_cache.insert(submodule.repository.clone(), Repository::open(&path)?);
                    subrepo_cache.get(&submodule.repository).unwrap()
                }
            };

            // Iterate commits of the submodule
            let mut subwalk = subrepo.revwalk()?;

            // If we know a previous commit of the same submodule from a previous main version,
            // then use that to stop the walk
            let last_commit = submodule_last_oid.get(&submodule.repository);
            if let Some(submodule_last) = last_commit {
                // If the submodule didn't change across versions, ignore the submodule for this
                // version.
                if submodule_last == &submodule.commit {
                    continue;
                }
                subwalk.push_range(&format!("{submodule_last}..{}", submodule.commit))?;
            } else {
                subwalk.push(submodule.commit)?;
            }
            submodule_last_oid.insert(submodule.repository.clone(), submodule.commit);

            let mut commits = vec![];
            for commit in subwalk {
                let commit = commit?;
                commits.push(commit);
            }
            submodule_commits.push((submodule, commits));
        }

        // If we encounter multiple commits across different versions for some reason,
        // we always attribute them to the earliest version that encountered the commit.
        // Since we iterate versions from oldest to newest, the retain below ensures that.
        version_commits.retain(|c| seen_commits.insert(*c));
        for (_, commits) in &mut submodule_commits {
            commits.retain(|c| seen_commits.insert(*c));
        }

        by_version.insert(
            version.clone(),
            VersionCommits {
                main: version_commits,
                submodules: submodule_commits,
            },
        );
    }

    // Validation: walk all commits and ensure that we saw them previously
    let head = versions.last().unwrap().commit;
    let mut walk = repo.revwalk()?;
    walk.push(head)?;
    for commit in walk {
        let commit = commit?;
        assert!(
            seen_commits.contains(&commit),
            "Commit {commit} was not visited"
        );
    }
    let submodules = get_submodules(repo, &repo.find_commit(head)?)?;
    for submodule in submodules {
        let repo = subrepo_cache
            .get(&submodule.repository)
            .expect("Submodule repository not found");
        let mut walk = repo.revwalk()?;
        walk.push(submodule.commit)?;
        for commit in walk {
            let commit = commit?;
            assert!(
                seen_commits.contains(&commit),
                "Submodule {} commit {commit} was not visited",
                submodule.repository
            );
        }
    }

    Ok(by_version)
}

/// This functions checks out all known submodules in parallel, to make it faster.
///
/// It walks through all versions and gathers the set of all submodules known in the history
/// of the main `repo`. Then it checks them out in parallel.
fn checkout_all_submodules(
    repo: &Repository,
    versions: &[VersionTag],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_submodules: HashSet<String> = HashSet::new();

    // Note that we have to walk all versions, because in the latest version of the repo, some past
    // submodules might already be removed.
    // It is still possible that we will later run into a submodule that was only present
    // *in-between* two versions, but that should be rare, and if it happens, it will be just
    // checked out serially later.
    for version in versions {
        let modules = get_submodules(repo, &repo.find_commit(version.commit)?)?;
        all_submodules.extend(modules.into_iter().map(|module| module.repository));
    }
    let mut submodules: Vec<String> = all_submodules.into_iter().collect();
    submodules.sort();

    let (tx, rx) = std::sync::mpsc::channel();
    let (res_tx, res_rx) = std::sync::mpsc::channel();

    // Submit work
    for submodule in &submodules {
        tx.send(submodule.clone())?;
    }
    drop(tx);

    // git is already partially parallel internally, and cloning involves network operations
    // Spawn a low number of threads to avoid oversubscribing
    let thread_count = 4;

    let rx = Arc::new(Mutex::new(rx));
    std::thread::scope(|scope| {
        for _ in 0..thread_count {
            let rx = rx.clone();
            let res_tx = res_tx.clone();
            scope.spawn(move || {
                loop {
                    let submodule = {
                        let Ok(msg) = rx.lock().unwrap().recv() else {
                            break;
                        };
                        msg
                    };
                    update_repo(&submodule)
                        .unwrap_or_else(|e| panic!("Cannot checkout submodule {submodule}: {e:?}"));
                    res_tx.send(()).unwrap();
                }
            });
        }
    });

    // Wait for all submodules to be checked out
    for _ in submodules {
        res_rx.recv()?;
    }
    Ok(())
}

#[derive(clap::ValueEnum, Clone)]
enum OutputMode {
    Html,
    Csv,
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(Self::Html),
            "csv" => Ok(Self::Csv),
            _ => Err(format!(
                "Invalid output mode {s}. Possible values: `html` or `csv`."
            )),
        }
    }
}

/// Primary entrypoint to generate and render the thanks information.
///
/// Thanks information will be rendered for
/// * each version identified by [`get_versions()`]
/// * the unreleased version "Beta"
/// * the unreleased version "Nightly"
/// * "all time" contributions across any of those versions
fn run(mode: OutputMode, mailmap_path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let by_version = generate_thanks(mailmap_path)?;
    let by_version: BTreeMap<_, _> = by_version
        .into_iter()
        .map(|(k, v)| (k, AuthorsWithScores::new(v)))
        .collect();

    let mut all_time = by_version.values().next().unwrap().authors.clone();
    for authors in by_version.values().skip(1) {
        all_time.extend(authors.authors.clone());
    }
    let all_time = AuthorsWithScores::new(all_time);

    match mode {
        OutputMode::Html => {
            site::render(by_version, all_time)?;
        }
        OutputMode::Csv => {
            use std::io::Write;

            let write = |path: &Path,
                         authors: AuthorsWithScores|
             -> Result<(), Box<dyn std::error::Error>> {
                let mut file = BufWriter::new(std::fs::File::create(path)?);
                for score in authors.scores {
                    let AuthorScore {
                        rank,
                        author,
                        email,
                        commits,
                    } = score;
                    writeln!(file, "{rank},{author},{email},{commits}")?;
                }
                Ok(())
            };

            let directory = PathBuf::from("output/csv");
            std::fs::create_dir_all(&directory)?;
            for (version, authors) in by_version {
                write(&directory.join(format!("{version}.csv")), authors)?;
            }
            write(&directory.join("all-time.csv"), all_time)?;
        }
    }

    Ok(())
}

#[derive(clap::Parser)]
struct Args {
    /// Output mode to use.
    #[arg(default_value = "html")]
    mode: OutputMode,

    /// Path to a .mailmap file.
    /// Can be used to test mailmap changes before committing them to the main repo.
    #[arg(long)]
    mailmap_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args.mode, args.mailmap_path) {
        eprintln!("Error: {}", err);
        let mut cur = &*err;
        while let Some(cause) = cur.source() {
            eprintln!("\tcaused by: {}", cause);
            cur = cause;
        }
        std::mem::drop(err);
        std::process::exit(1);
    }
}

/// A submodule that is used in a parent repository.
#[derive(Debug)]
struct Submodule {
    /// The commit of the submodule.
    commit: Oid,
    /// The URL of the submodule.
    repository: String,
}

/// Identify the submodules present in a repository as-of the given commit.
///
/// The actual submodules are identified based on [`modules_file()`]. These
/// are then filtered to only include submodules where the source URL includes
/// "rust-lang" or "rust-lang-nursery", and also filtered to excluded a few
/// specific repositories.
///
/// The returned [`Submodule`] objects include not only the repository the
/// submodule was cloned from, but also the commit *of the submodule* present
/// in the primary repository *as of the given `at` commit*. This information is
/// used to include thanks information for contributions to submodules.
fn get_submodules(
    repo: &Repository,
    at: &Commit,
) -> Result<Vec<Submodule>, Box<dyn std::error::Error>> {
    let submodule_cfg = modules_file(repo, at)?;
    let submodule_cfg = Config::parse(&submodule_cfg)?;
    let mut path_to_url = HashMap::new();
    let entries = submodule_cfg.entries(None)?;
    for entry in &entries {
        let entry = entry?;
        let name = entry.name().unwrap();
        if name.ends_with(".path") {
            let url = name.replace(".path", ".url");
            let url = submodule_cfg.get_string(&url).unwrap();
            path_to_url.insert(entry.value().unwrap().to_owned(), url);
        }
    }
    let mut submodules = Vec::new();
    let tree = at.tree()?;
    for (path, url) in &path_to_url {
        let path = Path::new(&path);
        let entry = tree.get_path(path);
        // the submodule may not actually exist
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        assert_eq!(entry.kind().unwrap(), git2::ObjectType::Commit);
        submodules.push(Submodule {
            commit: entry.id(),
            repository: url.to_owned(),
        });
    }
    submodules.retain(|s| {
        let is_rust =
            s.repository.contains("rust-lang") || s.repository.contains("rust-lang-nursery");
        let exclude = vec![
            "https://github.com/rust-lang/llvm.git",
            "https://github.com/rust-lang/llvm-project.git",
            "https://github.com/rust-lang/lld.git",
            "https://github.com/rust-lang/enzyme.git",
            "https://github.com/rust-lang-nursery/clang.git",
            "https://github.com/rust-lang-nursery/lldb.git",
            "https://github.com/rust-lang/libuv.git",
            "https://github.com/rust-lang/gyp.git",
            "https://github.com/rust-lang/jemalloc.git",
            "https://github.com/rust-lang/compiler-rt.git",
            "https://github.com/rust-lang/hoedown.git",
            "https://github.com/rust-lang/gcc.git",
        ];
        let repo_name = s.repository.to_lowercase();
        is_rust
            && !exclude.contains(&repo_name.as_str())
            && !exclude.contains(&&*format!("{}.git", repo_name))
    });

    // Sort the submodules to ensure deterministic commit iteration order
    submodules.sort_by(|a, b| a.repository.cmp(&b.repository));
    Ok(submodules)
}

/// Extract the contents of a `.gitmodules` file as of a specific commit.
///
/// If the file does not exist as of the given commit, an empty string is
/// returned in the result instead.
fn modules_file(repo: &Repository, at: &Commit) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(modules) = at.tree()?.get_name(".gitmodules") {
        Ok(String::from_utf8(
            modules.to_object(repo)?.peel_to_blob()?.content().into(),
        )?)
    } else {
        Ok(String::new())
    }
}
