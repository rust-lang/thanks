use git2::{Commit, Oid, Repository};
use semver::Version;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{cmp, fmt, str};

use config::Config;
use mailmap::{Author, Mailmap};

mod config;
mod mailmap;
mod site;

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
        git(&["-C", &path_s, "fetch", "origin", "master:master"])?;
    } else {
        git(&["clone", "--bare", &url, &path_s])?;
    }
    Ok(path)
}

struct VersionTag {
    version: Version,
    raw_tag: String,
    commit: Oid,
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
                    version: v,
                    raw_tag: tag.clone(),
                    commit: repo
                        .revparse_single(&tag)
                        .unwrap()
                        .peel_to_commit()
                        .unwrap()
                        .id(),
                })
        })
        .collect::<Vec<_>>();
    versions.sort();
    Ok(versions)
}

fn build_author_map(
    repo: &Repository,
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
        let commit_author = Author::new(commit.author());
        let author = mailmap.canonicalize(&commit_author);
        author_map.add(author, oid);
    }
    Ok(author_map)
}

fn generate_thanks() -> Result<BTreeMap<Version, AuthorMap>, Box<dyn std::error::Error>> {
    let path = update_repo("https://github.com/rust-lang/rust.git")?;
    let repo = git2::Repository::open(&path)?;
    let mailmap = Mailmap::read_from_repo(&repo)?;
    let mailmap = Mailmap::from_str(&mailmap)?;

    let versions = get_versions(&repo)?;

    let mut version_map = BTreeMap::new();

    for (idx, version) in versions.iter().enumerate() {
        let previous = if let Some(v) = idx.checked_sub(1).map(|idx| &versions[idx]) {
            v
        } else {
            let author_map = build_author_map(&repo, &mailmap, "", &version.raw_tag)?;
            version_map.insert(version.version.clone(), author_map);
            continue;
        };

        eprintln!("Processing {:?} to {:?}", previous, version);

        let mut modules: HashMap<PathBuf, (Option<&Submodule>, Option<&Submodule>)> =
            HashMap::new();
        let previous_commit = repo.find_commit(previous.commit)?;
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

        let mut author_map =
            build_author_map(&repo, &mailmap, &previous.raw_tag, &version.raw_tag)?;

        for (path, (pre, cur)) in &modules {
            match (pre, cur) {
                (Some(previous), Some(current)) => {
                    let subrepo = Repository::open(&path)?;
                    let submap = build_author_map(
                        &subrepo,
                        &mailmap,
                        &previous.commit.to_string(),
                        &current.commit.to_string(),
                    )?;
                    author_map.extend(submap);
                }
                (None, Some(current)) => {
                    let subrepo = Repository::open(&path)?;
                    let submap =
                        build_author_map(&subrepo, &mailmap, "", &current.commit.to_string())?;
                    author_map.extend(submap);
                }
                (None, None) => unreachable!(),
                (Some(_), None) => {
                    // nothing, if submodule was deleted then we presume no changes since then
                    // affect us
                }
            }
        }

        version_map.insert(version.version.clone(), author_map);
    }

    Ok(version_map)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let by_version = generate_thanks()?;
    let mut all_time = by_version.values().next().unwrap().clone();
    for map in by_version.values().skip(1) {
        all_time.extend(map.clone());
    }

    site::render(by_version, all_time)?;

    Ok(())
}

fn traverse_entry(
    r: &str,
    entry: &git2::TreeEntry,
    _repo: &Repository,
    path_to_url: &HashMap<String, String>,
) -> Result<Option<Submodule>, Box<dyn std::error::Error>> {
    if entry.kind().unwrap() == git2::ObjectType::Commit {
        let path_s = format!("{}{}", r, entry.name().unwrap());
        let path = PathBuf::from(&path_s);
        return Ok(Some(Submodule {
            path,
            commit: entry.id(),
            repository: path_to_url[&path_s].to_owned(),
        }));
    }
    Ok(None)
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
    let mut err = None;
    let mut submodules = Vec::new();
    at.tree()?.walk(
        git2::TreeWalkMode::PreOrder,
        |r, entry| match traverse_entry(r, entry, repo, &path_to_url) {
            Ok(Some(submodule)) => {
                if !submodule.repository.contains("llvm")
                    && !submodule.repository.contains("clang")
                    && submodule.repository.contains("rust")
                {
                    submodules.push(submodule);
                }
                git2::TreeWalkResult::Ok
            }
            Ok(None) => git2::TreeWalkResult::Ok,
            Err(e) => {
                err = Some(e);
                git2::TreeWalkResult::Abort
            }
        },
    )?;
    if let Some(err) = err {
        panic!("tree walk failed: {:?}", err);
    }
    Ok(submodules)
}

fn modules_file(repo: &Repository, at: &Commit) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(
        at.tree()?
            .get_name(".gitmodules")
            .unwrap()
            .to_object(&repo)?
            .peel_to_blob()?
            .content()
            .into(),
    )?)
}
