use crate::analyse::{AuthorMap, AuthorsWithScores, ProjectData};
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

    validate_homepage(projects);

    create_dir(root_dir)?;
    copy_public_assets(root_dir)?;

    let hb = hb()?;
    render_about_page(&hb, root_dir)?;

    let mut combined_all_time = AuthorMap::new();
    for data in projects {
        let project_out_dir = root_dir.join(data.display_config.url_path());
        create_dir(&project_out_dir)?;

        if data.display_config.is_homepage() {
            assert!(!data.display_config.is_versionless());
            render_project_index_page(&hb, data, root_dir)?;
        }

        if data.display_config.is_versionless() {
            render_all_time_page(&hb, data, &project_out_dir)?;
        } else {
            render_project_index_page(&hb, data, &project_out_dir)?;
            render_release_pages(&hb, data, &project_out_dir)?;
            render_all_time_page(&hb, data, &project_out_dir.join("all-time"))?;
        }

        combined_all_time.extend(data.all_time.authors.clone());
    }

    // Render combined all time data
    let combined_all_time = AuthorsWithScores::new(combined_all_time);
    let res = hb.render(
        "stats",
        &Release {
            common: CommonData::new("All-time upstream Rust Contributors".to_string()),
            release_title: String::from("All-time"),
            release: "the Rust toolchain".to_string(),
            count: combined_all_time.scores.len(),
            scores: &combined_all_time.scores,
            in_progress: true,
            is_homepage_project: true,
        },
    )?;

    fs::write(root_dir.join("all-time.html"), res)?;

    render_projects_page(&hb, projects, &combined_all_time, root_dir)?;

    Ok(())
}

/// Validate that there is exactly one homepage project
fn validate_homepage(projects: &[ProjectData]) {
    let mut homepage_project = None;
    for data in projects {
        if data.display_config.is_homepage() {
            if let Some(other) = homepage_project {
                panic!(
                    "Multiple projects that are marked as a homepage project: {} and {other}",
                    data.display_config.name()
                );
            }
            homepage_project = Some(data.display_config.name().to_string());
        }
    }
    if homepage_project.is_none() {
        eprintln!(
            "Warning: no rendered project is marked as homepage project, the index page will be missing"
        );
    }
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

fn render_project_index_page(
    hb: &Handlebars<'_>,
    data: &ProjectData,
    output_dir: &Path,
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
        name: &'static str,
        releases: Vec<Release>,
    }

    let mut releases = Vec::new();
    releases.push(Release {
        name: "All time".into(),
        url: format!("/{}/all-time/", data.display_config.url_path()),
        people: data.all_time.authors.iter().count(),
        commits: data.all_time.authors.iter().map(|(_, count)| count).sum(),
    });
    for (version, stats) in data.by_version.iter().rev() {
        releases.push(Release {
            name: version.name.clone(),
            url: format!("/{}/{}/", data.display_config.url_path(), version.version),
            people: stats.authors.iter().count(),
            commits: stats.authors.iter().map(|(_, count)| count).sum(),
        });
    }

    let res = hb.render(
        "index",
        &Index {
            common: CommonData::new(format!("{} Contributors", data.display_config.name()))
                .without_thanks_in_logo(),
            name: data.display_config.name(),
            releases,
        },
    )?;

    fs::write(output_dir.join("index.html"), res)?;
    Ok(())
}

fn render_about_page(
    hb: &Handlebars<'_>,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct About {
        common: CommonData,
    }

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

fn render_projects_page(
    hb: &Handlebars<'_>,
    projects: &[ProjectData],
    all_time: &AuthorsWithScores,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct ProjectInfo {
        name: String,
        link: String,
        people: usize,
        commits: usize,
    }

    #[derive(serde::Serialize)]
    struct Projects {
        common: CommonData,
        projects: Vec<ProjectInfo>,
        all_time_people: usize,
        all_time_commits: usize,
    }

    let projects: Vec<ProjectInfo> = projects
        .iter()
        .map(|data| ProjectInfo {
            name: data.display_config.name().to_string(),
            link: data.display_config.url_path().to_string(),
            people: data.all_time.authors.iter().count(),
            commits: data.all_time.authors.iter().map(|(_, count)| count).sum(),
        })
        .collect();

    let res = hb.render(
        "projects",
        &Projects {
            common: CommonData::new("Rust toolchain projects".into()),
            projects,
            all_time_people: all_time.authors.iter().count(),
            all_time_commits: all_time.authors.iter().map(|(_, count)| count).sum(),
        },
    )?;

    fs::write(output_dir.join("projects.html"), res)?;
    Ok(())
}

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

fn render_all_time_page(
    hb: &Handlebars<'_>,
    data: &ProjectData,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let scores = &data.all_time.scores;
    let res = hb.render(
        "stats",
        &Release {
            common: CommonData::new(format!(
                "All-time {} Contributors",
                data.display_config.name()
            )),
            release_title: String::from("All-time"),
            release: data.display_config.name().to_string(),
            count: scores.len(),
            scores,
            in_progress: true,
            is_homepage_project: data.display_config.is_homepage(),
        },
    )?;

    create_dir(output_dir)?;
    fs::write(output_dir.join("index.html"), res)?;
    Ok(())
}

fn render_release_pages(
    hb: &Handlebars<'_>,
    data: &ProjectData,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for (version, map) in &data.by_version {
        let scores = &map.scores;
        let res = hb.render(
            "stats",
            &Release {
                common: CommonData::new(format!(
                    "{} {version} Contributors",
                    data.display_config.name()
                )),
                release_title: version.name.clone(),
                release: version.to_string(),
                count: scores.len(),
                scores,
                in_progress: version.in_progress,
                is_homepage_project: data.display_config.is_homepage(),
            },
        )?;

        let version_dir = output_dir.join(version.to_string());
        create_dir(&version_dir)?;
        fs::write(version_dir.join("index.html"), res)?;
    }
    Ok(())
}
