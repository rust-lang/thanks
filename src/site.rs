use crate::{AuthorMap, VersionTag};
use handlebars::Handlebars;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

pub fn render(
    by_version: BTreeMap<VersionTag, AuthorMap>,
    all_time_map: AuthorMap,
) -> Result<(), Box<dyn std::error::Error>> {
    copy_public()?;
    index(&all_time_map, &by_version)?;
    about()?;
    releases(&by_version, &all_time_map)?;

    Ok(())
}

#[derive(serde::Serialize)]
struct CommonData {
    title: String,
    show_thanks_in_logo: bool,
}

impl CommonData {
    fn new(title: String) -> Self {
        CommonData {
            title,
            show_thanks_in_logo: true,
        }
    }

    fn without_thanks_in_logo(mut self) -> Self {
        self.show_thanks_in_logo = false;
        self
    }
}

fn hb() -> Result<Handlebars<'static>, Box<dyn std::error::Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_templates_directory(".hbs", "templates")?;
    Ok(handlebars)
}

fn create_dir<P: AsRef<Path>>(p: P) -> Result<(), std::io::Error> {
    match fs::create_dir_all(p) {
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
    all_time: &AuthorMap,
    by_version: &BTreeMap<VersionTag, AuthorMap>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release {
        name: String,
        url: String,
        people: usize,
        commits: usize,
    }
    #[derive(serde::Serialize)]
    struct Index {
        common: CommonData,
        releases: Vec<Release>,
    }
    let hb = hb()?;

    let mut releases = Vec::new();
    releases.push(Release {
        name: "All time".into(),
        url: "/rust/all-time/".into(),
        people: all_time.iter().count(),
        commits: all_time.iter().map(|(_, count)| count).sum(),
    });
    for (version, stats) in by_version.iter().rev() {
        releases.push(Release {
            name: version.name.clone(),
            url: format!("/rust/{}/", version.version),
            people: stats.iter().count(),
            commits: stats.iter().map(|(_, count)| count).sum(),
        });
    }

    let res = hb.render(
        "index",
        &Index {
            common: CommonData::new("Rust Contributors".into()).without_thanks_in_logo(),
            releases,
        },
    )?;

    fs::write("output/index.html", res)?;
    Ok(())
}

fn about() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct About {
        common: CommonData,
    }
    let hb = hb()?;

    let res = hb.render(
        "about",
        &About {
            common: CommonData::new("About - Rust Contributors".into()),
        },
    )?;

    create_dir("output/about")?;
    fs::write("output/about/index.html", res)?;
    Ok(())
}

#[derive(serde::Serialize, Ord, PartialOrd, Eq, PartialEq)]
struct Entry {
    rank: u32,
    author: String,
    email: String,
    commits: usize,
}

fn author_map_to_scores(map: &AuthorMap) -> Vec<Entry> {
    let debug_emails = std::env::var("DEBUG_EMAILS").is_ok_and(|value| value == "1");

    let scores = map
        .iter()
        .map(|(author, commits)| {
            let name = author.name.to_string();

            Entry {
                rank: 0,
                author: if debug_emails {
                    format!("{name} ({})", author.email)
                } else {
                    name
                },
                email: author.email.to_string(),
                commits,
            }
        })
        .collect::<Vec<_>>();
    let mut scores = deduplicate_scores(scores);
    scores.sort_by_key(|e| (std::cmp::Reverse(e.commits), e.author.clone()));

    let mut last_rank = 1;
    let mut ranked_at_current = 0;
    let mut last_commits = usize::max_value();
    for entry in &mut scores {
        if entry.commits < last_commits {
            last_commits = entry.commits;
            last_rank += ranked_at_current;
            ranked_at_current = 1;
        } else {
            ranked_at_current += 1;
        }
        entry.rank = last_rank;
    }
    scores
}

/// Deduplicate scores based on the assumption that an e-mail uniquely identifies a given
/// person. If there are multiple entries with the same email, their commit counts will be
/// merged into a single entry, with the canonical name being chosen based on the entry with
/// the most commits.
fn deduplicate_scores(entries: Vec<Entry>) -> Vec<Entry> {
    let mut entry_map: HashMap<String, Vec<Entry>> = HashMap::with_capacity(entries.len());
    for entry in entries {
        entry_map
            .entry(entry.email.clone())
            .or_default()
            .push(entry);
    }

    entry_map
        .into_values()
        .map(|mut entry| {
            // If there are multiple entries with the same maximum commit count, ensure that
            // the ordering is stable, by sorting based on the whole entry.
            entry.sort();
            let canonical_entry = entry.iter().max_by_key(|entry| entry.commits).unwrap();
            Entry {
                rank: 0,
                author: canonical_entry.author.clone(),
                email: canonical_entry.email.clone(),
                commits: entry.iter().map(|e| e.commits).sum(),
            }
        })
        .collect()
}

fn releases(
    by_version: &BTreeMap<VersionTag, AuthorMap>,
    all_time: &AuthorMap,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release {
        common: CommonData,
        release_title: String,
        release: String,
        count: usize,
        scores: Vec<Entry>,
        in_progress: bool,
    }
    let hb = hb()?;
    let scores = author_map_to_scores(&all_time);

    let res = hb.render(
        "stats",
        &Release {
            common: CommonData::new("All-time Rust Contributors".into()),
            release_title: String::from("All-time"),
            release: String::from("all of Rust"),
            count: scores.len(),
            scores,
            in_progress: true,
        },
    )?;

    create_dir("output/rust/all-time")?;
    fs::write("output/rust/all-time/index.html", res)?;

    for (version, map) in by_version {
        let scores = author_map_to_scores(&map);
        let res = hb.render(
            "stats",
            &Release {
                common: CommonData::new(format!("Rust {} Contributors", version)),
                release_title: version.name.clone(),
                release: version.to_string(),
                count: scores.len(),
                scores,
                in_progress: version.in_progress,
            },
        )?;

        create_dir(format!("output/rust/{}", version))?;
        fs::write(format!("output/rust/{}/index.html", version), res)?;
    }
    Ok(())
}
