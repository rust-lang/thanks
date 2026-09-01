use crate::git::VersionTag;
use crate::score::AuthorScore;
use crate::{AuthorMap, AuthorsWithScores};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn render(
    by_version: BTreeMap<VersionTag, AuthorsWithScores>,
    all_time_map: AuthorsWithScores,
) -> Result<(), Box<dyn std::error::Error>> {
    copy_public()?;
    index(&all_time_map.authors, &by_version)?;
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
        Err(e) => return Err(e),
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
            create_dir(Path::new("output").join(entry.path().strip_prefix("public/")?))?;
        }
    }
    Ok(())
}

fn index(
    all_time: &AuthorMap,
    by_version: &BTreeMap<VersionTag, AuthorsWithScores>,
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
            people: stats.authors.iter().count(),
            commits: stats.authors.iter().map(|(_, count)| count).sum(),
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

fn releases(
    by_version: &BTreeMap<VersionTag, AuthorsWithScores>,
    all_time: &AuthorsWithScores,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release<'a> {
        common: CommonData,
        release_title: String,
        release: String,
        count: usize,
        scores: &'a [AuthorScore],
        in_progress: bool,
    }
    let hb = hb()?;

    let scores = &all_time.scores;
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
        let scores = &map.scores;
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
