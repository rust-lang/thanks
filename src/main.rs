use git2::{Commit, Oid, Repository};
use semver::Version;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{cmp, fmt, str};

use config::Config;
use mailmap::{Author, Mailmap};

mod config;
mod mailmap;

fn git(args: &[&str]) -> String {
    let mut cmd = Command::new("git");
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    eprintln!("running {:?}", cmd);
    let out = cmd.spawn();
    let mut out = match out {
        Ok(v) => v,
        Err(err) => {
            panic!("Failed to spawn command `{:?}`: {:?}", cmd, err);
        }
    };

    let status = out.wait().expect("waited");

    assert!(
        status.success(),
        "failed to run `git {:?}`: {:?}",
        args,
        status
    );

    let mut stdout = Vec::new();
    out.stdout.unwrap().read_to_end(&mut stdout).unwrap();
    String::from_utf8_lossy(&stdout).into_owned()
}

lazy_static::lazy_static! {
    static ref UPDATED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn update_repo(slug: &str) -> PathBuf {
    let path_s = format!("repos/{}", slug);
    let path = PathBuf::from(&path_s);
    if !UPDATED.lock().unwrap().insert(slug.to_string()) {
        return path;
    }
    if path.exists() {
        git(&["-C", &path_s, "fetch", "origin", "master:master"]);
    } else {
        git(&[
            "clone",
            "--bare",
            &format!("https://github.com/{}.git", slug),
            &path_s,
        ]);
    }
    path
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
            Version::parse(&tag).ok().map(|v| VersionTag {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = update_repo("rust-lang/rust");
    let repo = git2::Repository::open(&path)?;
    let mailmap = Mailmap::read_from_repo(&path)?;
    let mailmap = Mailmap::from_str(&mailmap)?;

    let versions = get_versions(&repo)?;

    for (idx, version) in versions.iter().enumerate() {
        let previous = if let Some(v) = idx.checked_sub(1).map(|idx| &versions[idx]) {
            v
        } else {
            continue;
        };

        let mut walker = repo.revwalk()?;
        walker.push_range(&format!("{}..{}", previous.raw_tag, version.raw_tag))?;

        eprintln!("submodules for {:?}", previous);
        let previous_commit = repo.find_commit(previous.commit)?;
        let previous_modules = get_submodules(&repo, &previous_commit)?;
        for module in &previous_modules {
            update_repo(to_slug(&module.repository));
        }

        let mut author_map = HashMap::new();
        for oid in walker {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let commit_author = Author::new(commit.author());
            let author = mailmap.canonicalize(&commit_author);
            let entry = author_map.entry(author).or_insert(0);
            *entry += 1;
        }
    }

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
                submodules.push(submodule);
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

fn to_slug(mut url: &str) -> &str {
    let prefix = "https://github.com/";
    assert!(url.starts_with(prefix), "{}", url);
    url = &url[prefix.len()..];
    let suffix = ".git";
    if url.ends_with(suffix) {
        url = &url[..url.len() - suffix.len()];
    }
    url
}
