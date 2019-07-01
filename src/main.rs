use git2::{Commit, Oid, Repository};
use regex::{Regex, RegexBuilder};
use semver::Version;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{cmp, fmt, str};

use config::Config;
use mailmap::{Author, Mailmap};
use reviewers::Reviewers;

mod config;
mod error;
mod mailmap;
mod reviewers;
mod site;

use error::ErrorContext;

#[derive(Clone)]
pub struct AuthorMap {
    // author -> [commits]
    map: HashMap<Author, HashSet<Oid>>,
}

impl AuthorMap {
    fn new() -> Self {
        AuthorMap {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, author: Author, commit: Oid) {
        self.map
            .entry(author)
            .or_insert_with(HashSet::new)
            .insert(commit);
    }

    fn iter(&self) -> impl Iterator<Item = (&Author, usize)> {
        self.map.iter().map(|(k, v)| (k, v.len()))
    }

    fn extend(&mut self, other: Self) {
        for (author, set) in other.map {
            self.map
                .entry(author)
                .or_insert_with(HashSet::new)
                .extend(set);
        }
    }
}

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
                &url,
                &path_s,
            ])?;
            std::fs::remove_dir_all(&tmp)?;
        }
    } else {
        git(&["clone", "--bare", &url, &path_s])?;
    }
    Ok(path)
}

fn should_update() -> bool {
    std::env::args_os().nth(1).unwrap_or_default() == "--refresh"
}

#[derive(Clone)]
pub struct VersionTag {
    name: String,
    version: Version,
    raw_tag: String,
    commit: Oid,
    in_progress: bool,
}

impl fmt::Display for VersionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)
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
        Some(self.cmp(&other))
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

fn get_versions(repo: &Repository) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
    let tags = repo
        .tag_names(None)?
        .into_iter()
        .filter_map(|v| v)
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();
    let mut versions = tags
        .iter()
        .filter_map(|tag| {
            Version::parse(&tag)
                .or_else(|_| Version::parse(&format!("{}.0", tag)))
                .ok()
                .map(|v| VersionTag {
                    name: format!("Rust {}", v),
                    version: v,
                    raw_tag: tag.clone(),
                    commit: repo
                        .revparse_single(&tag)
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
            if let Some(caps) = RE.captures(line) {
                coauthors.push(Author {
                    name: caps["name"].to_string(),
                    email: caps["email"].to_string(),
                });
            }
        }
    }
    coauthors
}

fn build_author_map(
    repo: &Repository,
    reviewers: &Reviewers,
    mailmap: &Mailmap,
    from: &str,
    to: &str,
) -> Result<AuthorMap, Box<dyn std::error::Error>> {
    match build_author_map_(repo, reviewers, mailmap, from, to) {
        Ok(o) => Ok(o),
        Err(err) => Err(ErrorContext(
            format!(
                "build_author_map(repo={}, from={:?}, to={:?})",
                repo.path().display(),
                from,
                to
            ),
            err,
        ))?,
    }
}

// Note this is not the bors merge commit of a rollup
fn is_rollup_commit(commit: &Commit) -> bool {
    let summary = commit.summary().unwrap_or("");
    summary.starts_with("Rollup merge of #")
}

fn parse_bors_reviewer(
    reviewers: &Reviewers,
    repo: &Repository,
    commit: &Commit,
) -> Option<Vec<Author>> {
    if commit.author().name_bytes() != b"bors" || commit.committer().name_bytes() != b"bors" {
        return None;
    }

    let to_author = |list: &str| -> Vec<Author> {
        list.trim_end_matches('.')
            .split(|c| c == ',' || c == '+')
            .map(|r| r.trim_start_matches('@'))
            .map(|r| r.trim_end_matches('`'))
            .map(|r| r.trim())
            .inspect(|r| {
                if !r
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_digit(10) || c == '-' || c == '_')
                {
                    panic!(
                        "to_author for {} contained non-alphabetic characters: {:?}",
                        commit.id(),
                        r
                    );
                }
            })
            .map(|r| reviewers.to_author(r))
            .collect::<Vec<_>>()
    };

    let message = commit.message().unwrap_or("");
    let mut reviewers = if let Some(line) = message.lines().find(|l| l.contains(" r=")) {
        let start = line.find("r=").unwrap() + 2;
        let end = line[start..]
            .find(' ')
            .map(|pos| pos + start)
            .unwrap_or(line.len());
        to_author(&line[start..end])
    } else if let Some(line) = message.lines().find(|l| l.starts_with("Reviewed-by: ")) {
        let line = &line["Reviewed-by: ".len()..];
        to_author(&line)
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
        return None;
    };
    reviewers.sort();
    reviewers.dedup();
    Some(reviewers)
}

fn build_author_map_(
    repo: &Repository,
    reviewers: &Reviewers,
    mailmap: &Mailmap,
    from: &str,
    to: &str,
) -> Result<AuthorMap, Box<dyn std::error::Error>> {
    let mut walker = repo.revwalk()?;
    if from == "" {
        let to = repo.revparse_single(to)?.peel_to_commit()?.id();
        walker.push(to)?;
    } else {
        walker.push_range(&format!("{}..{}", from, to))?;
    }

    let mut author_map = AuthorMap::new();
    for oid in walker {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let mut commit_authors = Vec::new();
        if !is_rollup_commit(&commit) {
            // We ignore the author of rollup-merge commits, and account for
            // that author once by counting the reviewer of all bors merges. For
            // rollups, we consider that this is the most relevant person, which
            // is usually the case.
            //
            // Otherwise, a single rollup with N PRs attributes N commits to the author of the
            // rollup, which isn't fair.
            commit_authors.push(Author::new(commit.author()));
        }
        if let Some(reviewers) = parse_bors_reviewer(&reviewers, &repo, &commit) {
            commit_authors.extend(reviewers);
        }
        commit_authors.extend(commit_coauthors(&commit));
        for author in commit_authors {
            let author = mailmap.canonicalize(&author);
            author_map.add(author, oid);
        }
    }
    Ok(author_map)
}

fn generate_thanks() -> Result<BTreeMap<VersionTag, AuthorMap>, Box<dyn std::error::Error>> {
    let path = update_repo("https://github.com/rust-lang/rust.git")?;
    let repo = git2::Repository::open(&path)?;
    let mailmap = Mailmap::from_repo(&repo)?;
    let reviewers = Reviewers::new();

    let mut versions = get_versions(&repo)?;
    versions.push(VersionTag {
        name: String::from("Beta"),
        version: {
            let mut last = versions.last().unwrap().version.clone();
            last.increment_minor();
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
        name: String::from("Master"),
        version: {
            // master is plus 1 minor versions off of beta, which we just pushed
            let mut last = versions.last().unwrap().version.clone();
            last.increment_minor();
            last
        },
        raw_tag: String::from("master"),
        commit: repo
            .revparse_single("master")
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id(),
        in_progress: true,
    });

    let mut version_map = BTreeMap::new();

    for (idx, version) in versions.iter().enumerate() {
        let previous = if let Some(v) = idx.checked_sub(1).map(|idx| &versions[idx]) {
            v
        } else {
            let author_map = build_author_map(&repo, &reviewers, &mailmap, "", &version.raw_tag)?;
            version_map.insert(version.clone(), author_map);
            continue;
        };

        eprintln!("Processing {:?} to {:?}", previous, version);

        let mut modules: HashMap<PathBuf, (Option<&Submodule>, Option<&Submodule>)> =
            HashMap::new();
        let previous_commit = repo.find_commit(previous.commit).map_err(|e| {
            ErrorContext(
                format!(
                    "find_commit: repo={}, commit={}",
                    repo.path().display(),
                    previous.commit
                ),
                Box::new(e),
            )
        })?;
        let previous_modules = get_submodules(&repo, &previous_commit)?;
        for module in &previous_modules {
            if let Ok(path) = update_repo(&module.repository) {
                modules.insert(path, (Some(module), None));
            }
        }

        let current_commit = repo.find_commit(version.commit)?;
        let current_modules = get_submodules(&repo, &current_commit)?;
        for module in &current_modules {
            if let Ok(path) = update_repo(&module.repository) {
                modules.entry(path).or_insert((None, None)).1 = Some(module);
            }
        }

        let mut author_map = build_author_map(
            &repo,
            &reviewers,
            &mailmap,
            &previous.raw_tag,
            &version.raw_tag,
        )
        .map_err(|e| ErrorContext(format!("From {} to {}", previous, version), e))?;

        for (path, (pre, cur)) in &modules {
            match (pre, cur) {
                (Some(previous), Some(current)) => {
                    let subrepo = Repository::open(&path)?;
                    let submap = build_author_map(
                        &subrepo,
                        &reviewers,
                        &mailmap,
                        &previous.commit.to_string(),
                        &current.commit.to_string(),
                    )?;
                    author_map.extend(submap);
                }
                (None, Some(current)) => {
                    let subrepo = Repository::open(&path)?;
                    let submap = build_author_map(
                        &subrepo,
                        &reviewers,
                        &mailmap,
                        "",
                        &current.commit.to_string(),
                    )?;
                    author_map.extend(submap);
                }
                (None, None) => unreachable!(),
                (Some(_), None) => {
                    // nothing, if submodule was deleted then we presume no changes since then
                    // affect us
                }
            }
        }

        version_map.insert(version.clone(), author_map);
    }

    Ok(version_map)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let by_version = generate_thanks()?;

    let mut all_time = by_version.values().next().unwrap().clone();
    for map in by_version.values().skip(1) {
        all_time.extend(map.clone());
    }

    site::render(by_version, all_time)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut cur = &*err;
        while let Some(cause) = cur.source() {
            eprintln!("\tcaused by: {}", cause);
            cur = cause;
        }
        std::mem::drop(cur);
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct Submodule {
    path: PathBuf,
    commit: Oid,
    // url
    repository: String,
}

fn get_submodules(
    repo: &Repository,
    at: &Commit,
) -> Result<Vec<Submodule>, Box<dyn std::error::Error>> {
    let submodule_cfg = modules_file(&repo, &at)?;
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
        let entry = tree.get_path(&path);
        // the submodule may not actually exist
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        assert_eq!(entry.kind().unwrap(), git2::ObjectType::Commit);
        submodules.push(Submodule {
            path: path.to_owned(),
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
            "https://github.com/rust-lang-nursery/clang.git",
            "https://github.com/rust-lang-nursery/lldb.git",
            "https://github.com/rust-lang/libuv.git",
            "https://github.com/rust-lang/gyp.git",
            "https://github.com/rust-lang/jemalloc.git",
            "https://github.com/rust-lang/compiler-rt.git",
            "https://github.com/rust-lang/hoedown.git",
        ];
        is_rust
            && !exclude.contains(&s.repository.as_str())
            && !exclude.contains(&&*format!("{}.git", s.repository))
    });
    Ok(submodules)
}

fn modules_file(repo: &Repository, at: &Commit) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(modules) = at.tree()?.get_name(".gitmodules") {
        Ok(String::from_utf8(
            modules.to_object(&repo)?.peel_to_blob()?.content().into(),
        )?)
    } else {
        return Ok(String::new());
    }
}
