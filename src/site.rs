use crate::AuthorMap;
use handlebars::Handlebars;
use semver::Version;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn render(
    by_version: BTreeMap<Version, AuthorMap>,
    all_time_map: AuthorMap,
) -> Result<(), Box<dyn std::error::Error>> {
    copy_public()?;
    index(&all_time_map, &by_version)?;
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
    all_time: &AuthorMap,
    by_version: &BTreeMap<Version, AuthorMap>,
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
        maintenance: bool,
        show_thanks_in_logo: bool,
        releases: Vec<Release>,
    }
    let hb = hb()?;

    let mut releases = Vec::new();
    releases.push(Release {
        name: "All the time".into(),
        url: "/all-time.html".into(),
        people: all_time.iter().count(),
        commits: all_time.iter().map(|(_, count)| count).sum(),
    });
    for (version, stats) in by_version.iter().rev() {
        releases.push(Release {
            name: format!("Rust {}", version),
            url: format!("/{}.html", version),
            people: stats.iter().count(),
            commits: stats.iter().map(|(_, count)| count).sum(),
        });
    }

    let res = hb.render(
        "index",
        &Index {
            maintenance: false,
            show_thanks_in_logo: false,
            releases,
        },
    )?;

    fs::write("output/index.html", res)?;
    Ok(())
}

fn about() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct About {
        maintenance: bool,
        show_thanks_in_logo: bool,
    }
    let hb = hb()?;

    let res = hb.render(
        "about",
        &About {
            maintenance: false,
            show_thanks_in_logo: true,
        },
    )?;

    fs::write("output/about.html", res)?;
    Ok(())
}

#[derive(serde::Serialize)]
struct Entry {
    rank: u32,
    author: String,
    commits: usize,
}

fn author_map_to_scores(map: &AuthorMap) -> Vec<Entry> {
    let mut scores = map
        .iter()
        .map(|(author, commits)| Entry {
            rank: 0,
            author: author.name.clone(),
            commits: commits,
        })
        .collect::<Vec<_>>();
    scores.sort_by_key(|e| std::cmp::Reverse((e.commits, e.author.clone())));
    let mut last_rank = 0;
    let mut last_commits = usize::max_value();
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
    by_version: &BTreeMap<Version, AuthorMap>,
    all_time: &AuthorMap,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release {
        maintenance: bool,
        show_thanks_in_logo: bool,
        release_title: String,
        release: String,
        count: usize,
        scores: Vec<Entry>,
    }
    let hb = hb()?;
    let scores = author_map_to_scores(&all_time);

    let res = hb.render(
        "stats",
        &Release {
            maintenance: false,
            show_thanks_in_logo: true,
            release_title: String::from("All-time"),
            release: String::from("all of Rust"),
            count: scores.len(),
            scores,
        },
    )?;

    fs::write("output/all-time.html", res)?;

    for (version, map) in by_version {
        let scores = author_map_to_scores(&map);
        let res = hb.render(
            "stats",
            &Release {
                maintenance: false,
                show_thanks_in_logo: true,
                release_title: version.to_string(),
                release: version.to_string(),
                count: scores.len(),
                scores,
            },
        )?;

        fs::write(format!("output/{}.html", version), res)?;
    }
    Ok(())
}
