use clap::Parser;
use git2::Repository;
use mailmap::Mailmap;
use reviewers::Reviewers;
use std::collections::{BTreeMap, HashSet};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::str;
use std::time::Instant;
use unicase::UniCase;

mod analyse;
mod config;
mod error;
mod git;
mod projects;
mod reviewers;
mod score;
mod site;

use crate::analyse::{
    AuthorMap, AuthorsWithScores, build_author_map, compute_data, gather_all_commits,
};
use crate::git::{VersionTag, mailmap_from_repo, update_repo};
use crate::projects::{CratesIo, DocsRs, Project, Rust, Rustup};
use crate::score::AuthorScore;

fn generate_thanks(
    project: &dyn Project,
    mailmap_path: Option<PathBuf>,
) -> Result<BTreeMap<VersionTag, AuthorMap>, Box<dyn std::error::Error>> {
    eprintln!("Generating {}", project.name());

    let path = update_repo(project.repo_url())?;
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

    let versions = project.get_versions(&repo)?;

    let start = Instant::now();

    let by_version = gather_all_commits(&repo, versions)?;

    let version_count = by_version.len();
    let commit_count = by_version.values().map(|v| v.total_commits()).sum::<u64>();
    eprintln!(
        "Gathered {version_count} versions with {commit_count} total commits in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    let ignored_emails: HashSet<UniCase<String>> = project
        .ignored_emails()
        .iter()
        .map(|email| UniCase::new(email.to_string()))
        .collect();

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

            author_map.retain(|author| !ignored_emails.contains(&author.email));

            Ok::<_, Box<dyn std::error::Error>>((version, author_map))
        })
        .collect::<Result<_, _>>()?;
    eprintln!(
        "Analyzed contributions in {:.2}s",
        start.elapsed().as_secs_f64()
    );

    Ok(version_map)
}

/// Return all projects for which we currently generate contribution statistics.
fn get_all_projects() -> Vec<Box<dyn Project>> {
    // When adding new projects here, be sure to also update templates/about.hbs
    vec![
        Box::new(Rust),
        Box::new(Rustup),
        Box::new(CratesIo),
        Box::new(DocsRs),
    ]
}

fn run(
    mode: OutputMode,
    mailmap_path: Option<PathBuf>,
    selected_project: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut projects = get_all_projects();
    if let Some(selected) = selected_project {
        projects.retain(|p| p.name().to_lowercase() == selected);
        if projects.is_empty() {
            panic!("No projects found with name {selected}");
        }
    }

    let mut data = vec![];
    for project in projects {
        data.push(compute_data(project, mailmap_path.clone())?);
    }

    match mode {
        OutputMode::Html => {
            site::render_projects(&data, Path::new("output"))?;
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

            let csv_dir = PathBuf::from("output/csv");
            for data in data {
                let directory = csv_dir.join(data.project.name().to_lowercase());
                std::fs::create_dir_all(&directory)?;
                for (version, authors) in data.by_version {
                    write(&directory.join(format!("{version}.csv")), authors)?;
                }
                write(&directory.join("all-time.csv"), data.all_time)?;
            }
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

    /// Render only the selected project.
    /// If not given, all projects are rendered.
    /// The value should correspond to the project's lowercase name.
    #[arg(long)]
    project: Option<String>,
}

#[derive(clap::ValueEnum, Copy, Clone)]
enum OutputMode {
    Html,
    Csv,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args.mode, args.mailmap_path, args.project.as_deref()) {
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
