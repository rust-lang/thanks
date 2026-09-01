use clap::Parser;
use git2::Repository;
use mailmap::Mailmap;
use reviewers::Reviewers;
use std::collections::BTreeMap;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::str;
use std::str::FromStr;
use std::time::Instant;

mod analyse;
mod config;
mod error;
mod git;
mod reviewers;
mod score;
mod site;

use crate::analyse::{AuthorMap, AuthorsWithScores, build_author_map, gather_all_commits};
use crate::git::{VersionTag, get_versions, mailmap_from_repo, update_repo};
use crate::score::AuthorScore;

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

    let mut versions = get_versions(&repo, "Rust")?;
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
