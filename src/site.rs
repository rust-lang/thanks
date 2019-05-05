use crate::mailmap::Author;
use handlebars::Handlebars;
use semver::Version;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

pub fn render(
    by_version: BTreeMap<Version, HashMap<Author, u32>>,
    all_time_map: HashMap<Author, u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    copy_public()?;
    index(&by_version)?;
    about()?;
    releases(&by_version, &all_time_map)?;

    Ok(())
}

fn hb() -> Result<Handlebars, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_templates_directory(".hbs", "templates")?;
    Ok(handlebars)
}

fn create_dir<P: AsRef<Path>>(p: P) -> Result<(), std::io::Error> {
    match fs::create_dir(p) {
        Ok(()) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e.into()),
    };
    Ok(())
}

fn copy_public() -> Result<(), Box<dyn std::error::Error>> {
    let wd = walkdir::WalkDir::new("public");
    fs::create_dir_all("output")?;
    for entry in wd {
        let entry = entry?;
        if entry.file_type().is_file() {
            fs::copy(
                entry.path(),
                Path::new("output").join(entry.path().strip_prefix("public/")?),
            )?;
        } else if entry.file_type().is_dir() {
            create_dir(&Path::new("output").join(entry.path().strip_prefix("public/")?))?;
        }
    }
    Ok(())
}

fn index(
    by_version: &BTreeMap<Version, HashMap<Author, u32>>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Index {
        maintenance: bool,
        releases: Vec<String>,
    }
    let hb = hb()?;

    let res = hb.render(
        "index",
        &Index {
            maintenance: false,
            releases: by_version.keys().rev().map(|v| v.to_string()).collect(),
        },
    )?;

    fs::write("output/index.html", res)?;
    Ok(())
}

fn about() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct About {
        maintenance: bool,
    }
    let hb = hb()?;

    let res = hb.render("about", &About { maintenance: false })?;

    create_dir("output/about")?;
    fs::write("output/about/index.html", res)?;
    Ok(())
}

#[derive(serde::Serialize)]
struct Entry {
    rank: u32,
    author: String,
    commits: u32,
}

fn author_map_to_scores(map: &HashMap<Author, u32>) -> Vec<Entry> {
    let mut scores = map
        .iter()
        .map(|(author, commits)| Entry {
            rank: 0,
            author: author.name.clone(),
            commits: *commits,
        })
        .collect::<Vec<_>>();
    scores.sort_by_key(|e| std::cmp::Reverse((e.commits, e.author.clone())));
    let mut last_rank = 0;
    let mut last_commits = u32::max_value();
    for entry in &mut scores {
        if entry.commits < last_commits {
            last_commits = entry.commits;
            last_rank += 1;
        }
        entry.rank = last_rank;
    }
    scores
}

fn releases(
    by_version: &BTreeMap<Version, HashMap<Author, u32>>,
    all_time: &HashMap<Author, u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release {
        maintenance: bool,
        release_title: String,
        release: String,
        count: usize,
        scores: Vec<Entry>,
    }
    let hb = hb()?;
    let scores = author_map_to_scores(&all_time);

    let res = hb.render(
        "all-time",
        &Release {
            maintenance: false,
            release_title: String::from("All-time"),
            release: String::from("all of Rust"),
            count: scores.len(),
            scores,
        },
    )?;

    fs::create_dir_all("output/rust/all-time")?;
    fs::write("output/rust/all-time/index.html", res)?;

    for (version, map) in by_version {
        let scores = author_map_to_scores(&map);
        let res = hb.render(
            "all-time",
            &Release {
                maintenance: false,
                release_title: version.to_string(),
                release: version.to_string(),
                count: scores.len(),
                scores,
            },
        )?;

        let dir = PathBuf::from(format!("output/rust/{}", version));
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("index.html"), res)?;
    }
    Ok(())
}
