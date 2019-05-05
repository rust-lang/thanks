use git2::Oid;
use semver::Version;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

use mailmap::{Author, Mailmap};

mod mailmap;

fn git(args: &[&str]) -> String {
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

fn update_repo(slug: &str) -> PathBuf {
    let path_s = format!("repos/{}", slug);
    let path = PathBuf::from(&path_s);
    if path.exists() {
        git(&["-C", &path_s, "pull"]);
    } else {
        git(&[
            "clone",
            &format!("https://github.com/{}.git", slug),
            &path_s,
        ]);
    }
    path
}

struct VersionTag {
    version: Version,
    raw_tag: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = update_repo("rust-lang/rust");
    let repo = git2::Repository::open(&path)?;
    let mailmap = Mailmap::read_from_repo(&path)?;
    let mailmap = Mailmap::from_str(&mailmap)?;

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
            })
        })
        .collect::<Vec<_>>();
    versions.sort();

    for (idx, version) in versions.iter().enumerate() {
        let previous = if let Some(v) = idx.checked_sub(1).map(|idx| &versions[idx]) {
            v
        } else {
            continue;
        };

        let mut walker = repo.revwalk()?;
        walker.push_range(&format!("{}..{}", previous.raw_tag, version.raw_tag))?;

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
