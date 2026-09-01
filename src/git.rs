use git2::{Oid, Repository};
use mailmap::Mailmap;
use semver::Version;
use std::collections::HashSet;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::{cmp, fmt};

/// Run a `git` command with the given arguments.
///
/// # Panics
///
/// Panics if the `git` command cannot be spawned
pub fn git(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
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
pub fn update_repo(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
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
pub fn get_versions(
    repo: &Repository,
    name_prefix: &str,
) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
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
                    name: format!("{name_prefix} {}", v),
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

/// Construct a `Mailmap` based on the latest commit in the given repository.
///
/// Returns an error if the latest commit cannot be retrieved or if it does not
/// contain a `.mailmap` file to read.
pub fn mailmap_from_repo(repo: &git2::Repository) -> Result<Mailmap, Box<dyn std::error::Error>> {
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

/// Information about a git tag or other reference to treat as a tag.
#[derive(Clone)]
pub struct VersionTag {
    /// Some custom name, e.g. "Rust 1.94.0" or "Beta".
    pub name: String,
    /// The parsed Version for this tag.
    pub version: Version,
    /// The raw name of the tag or commit.
    pub raw_tag: String,
    /// The commit for this tag or reference.
    pub commit: Oid,
    /// Whether this version is still being developed.
    ///
    /// This should only be true for the "Beta" and "Nightly" versions.
    pub in_progress: bool,
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
