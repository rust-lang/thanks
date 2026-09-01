use crate::analyse::ProjectData;
use crate::score::AuthorScore;
use handlebars::Handlebars;
use std::fs;
use std::path::Path;

pub fn render_projects(
    projects: &[ProjectData],
    root_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!(
        "Rendering {} project{} to HTML",
        projects.len(),
        if projects.len() == 1 { "" } else { "s" }
    );

    // Validate that there is exactly one homepage project
    let mut homepage_project = None;
    for data in projects {
        if data.project.is_homepage() {
            if let Some(other) = homepage_project {
                panic!(
                    "Multiple projects that are marked as a homepage project: {} and {other}",
                    data.project.name()
                );
            }
            homepage_project = Some(data.project.name().to_string());
        }
    }
    if homepage_project.is_none() {
        eprintln!(
            "Warning: no rendered project is marked as homepage project, the index page will be missing"
        );
    }

    create_dir(root_dir)?;

    copy_public_assets(root_dir)?;
    about(root_dir)?;

    for data in projects {
        let project_out_dir = root_dir.join(data.project.url_path());
        create_dir(&project_out_dir)?;

        let index_dir = if data.project.is_homepage() {
            root_dir
        } else {
            &project_out_dir
        };
        index(data, index_dir)?;
        releases(data, &project_out_dir)?;
    }

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

fn copy_public_assets(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let wd = walkdir::WalkDir::new("public");
    for entry in wd {
        let entry = entry?;
        if entry.file_type().is_file() {
            fs::copy(
                entry.path(),
                output_dir.join(entry.path().strip_prefix("public/")?),
            )?;
        } else if entry.file_type().is_dir() {
            create_dir(output_dir.join(entry.path().strip_prefix("public/")?))?;
        }
    }
    Ok(())
}

fn index(data: &ProjectData, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
        url: format!("/{}/all-time/", data.project.url_path()),
        people: data.all_time.authors.iter().count(),
        commits: data.all_time.authors.iter().map(|(_, count)| count).sum(),
    });
    for (version, stats) in data.by_version.iter().rev() {
        releases.push(Release {
            name: version.name.clone(),
            url: format!("/{}/{}/", data.project.url_path(), version.version),
            people: stats.authors.iter().count(),
            commits: stats.authors.iter().map(|(_, count)| count).sum(),
        });
    }

    let res = hb.render(
        "index",
        &Index {
            common: CommonData::new(format!("{} Contributors", data.project.name()))
                .without_thanks_in_logo(),
            releases,
        },
    )?;

    fs::write(output_dir.join("index.html"), res)?;
    Ok(())
}

fn about(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

    let about_dir = output_dir.join("about");
    create_dir(&about_dir)?;
    fs::write(about_dir.join("index.html"), res)?;
    Ok(())
}

fn releases(data: &ProjectData, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct Release<'a> {
        common: CommonData,
        release_title: String,
        release: String,
        count: usize,
        scores: &'a [AuthorScore],
        in_progress: bool,
        is_homepage_project: bool,
    }
    let hb = hb()?;

    let scores = &data.all_time.scores;
    let res = hb.render(
        "stats",
        &Release {
            common: CommonData::new(format!("All-time {} Contributors", data.project.name())),
            release_title: String::from("All-time"),
            release: format!("all of {}", data.project.name()),
            count: scores.len(),
            scores,
            in_progress: true,
            is_homepage_project: data.project.is_homepage(),
        },
    )?;

    let all_time_dir = output_dir.join("all-time");
    create_dir(&all_time_dir)?;
    fs::write(all_time_dir.join("index.html"), res)?;

    for (version, map) in &data.by_version {
        let scores = &map.scores;
        let res = hb.render(
            "stats",
            &Release {
                common: CommonData::new(format!("{} {version} Contributors", data.project.name())),
                release_title: version.name.clone(),
                release: version.to_string(),
                count: scores.len(),
                scores,
                in_progress: version.in_progress,
                is_homepage_project: data.project.is_homepage(),
            },
        )?;

        let version_dir = output_dir.join(version.to_string());
        create_dir(&version_dir)?;
        fs::write(version_dir.join("index.html"), res)?;
    }
    Ok(())
}
