use crate::error::ErrorContext;
use crate::git::{Submodule, VersionTag, update_repo};
use crate::projects::Project;
use crate::reviewers::Reviewers;
use crate::score::{AuthorScore, author_map_to_scores};
use crate::{generate_thanks, git, reviewers};
use git2::{Commit, Oid, Repository};
use mailmap::{Author, Mailmap};
use regex::{Regex, RegexBuilder};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Instant;

pub struct ProjectDisplayConfig {
    /// Name of the project, displayed on the website.
    name: &'static str,
    /// Path under which the project will be available on the web.
    /// If the `url` is e.g. `rust`, it will be available under `/rust/`.
    url_path: &'static str,
    /// Should this project be displayed as the main homepage project?
    is_homepage: bool,
    /// Returns true if the project does not track versions explicitly.
    /// It will be rendered as a single page with all-time contributions.
    is_versionless: bool,
}

impl ProjectDisplayConfig {
    /// Name of the project, displayed on the website.
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Path under which the project will be available on the web.
    /// If the `url` is e.g. `rust`, it will be available under `/rust/`.
    pub fn url_path(&self) -> &'static str {
        self.url_path
    }
    /// Should this project be displayed as the main homepage project?
    pub fn is_homepage(&self) -> bool {
        self.is_homepage
    }
    /// Returns true if the project does not track versions explicitly.
    /// It will be rendered as a single page with all-time contributions.
    pub fn is_versionless(&self) -> bool {
        self.is_versionless
    }
}

pub struct ProjectData {
    pub display_config: ProjectDisplayConfig,
    pub by_version: BTreeMap<VersionTag, AuthorsWithScores>,
    pub all_time: AuthorsWithScores,
}

pub fn compute_data<P: Project>(
    project: P,
    mailmap_path: Option<PathBuf>,
) -> Result<ProjectData, Box<dyn std::error::Error>> {
    let by_version = generate_thanks(&project, mailmap_path)?;
    let by_version: BTreeMap<_, _> = by_version
        .into_iter()
        .map(|(k, v)| (k, AuthorsWithScores::new(v)))
        .collect();

    let mut all_time = by_version.values().next().unwrap().authors.clone();
    for authors in by_version.values().skip(1) {
        all_time.extend(authors.authors.clone());
    }
    let all_time = AuthorsWithScores::new(all_time);
    Ok(ProjectData {
        display_config: ProjectDisplayConfig {
            name: project.name(),
            url_path: project.url_path(),
            is_homepage: project.is_homepage(),
            is_versionless: project.is_versionless(),
        },
        by_version,
        all_time,
    })
}

/// Gather all commits for the given versions from the given repository, including all of its
/// submodules.
/// The commits are grouped by the individual versions.
pub fn gather_all_commits(
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

fn get_submodules(
    repo: &Repository,
    at: &Commit,
) -> Result<Vec<Submodule>, Box<dyn std::error::Error>> {
    let mut submodules = git::get_submodules(repo, at)?;
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
    Ok(submodules)
}

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
    pub fn new() -> Self {
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
    pub fn iter(&self) -> impl Iterator<Item = (&Author, usize)> {
        self.map.iter().map(|(k, v)| (k, v.len()))
    }

    /// Keep only authors who pass the given filter.
    pub fn retain<F>(&mut self, filter: F)
    where
        F: Fn(&Author) -> bool,
    {
        self.map.retain(|author, _| filter(author));
    }

    /// Merge in the authorship data from another instance.
    pub fn extend(&mut self, other: Self) {
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
    pub fn new(authors: AuthorMap) -> Self {
        let scores = author_map_to_scores(&authors);
        Self { authors, scores }
    }
}

pub struct VersionCommits {
    pub main: Vec<Oid>,
    pub submodules: Vec<(Submodule, Vec<Oid>)>,
}

impl VersionCommits {
    pub fn total_commits(&self) -> u64 {
        self.main.len() as u64
            + self
                .submodules
                .iter()
                .map(|(_, commits)| commits.len() as u64)
                .sum::<u64>()
    }
}

/// Build up an [`AuthorMap`] of commits authored between `from` and `to`.
///
/// This function is a wrapper around [`build_author_map_`] to add additional
/// context to any errors; see that function for further documentation.
pub fn build_author_map(
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

/// Identify the co-authors, if any, of a commit
///
/// Co-authors are determined based on the commit message having lines starting
/// `Co-authored-by: ` followed by a name and then an email enclosed in `<>`.
fn commit_coauthors(commit: &Commit) -> Vec<Author> {
    let mut coauthors = vec![];
    if let Some(msg) = commit.message_raw() {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            RegexBuilder::new(r"^Co-authored-by: (?P<name>.*) <(?P<email>.*)>")
                .case_insensitive(true)
                .build()
                .unwrap()
        });

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
