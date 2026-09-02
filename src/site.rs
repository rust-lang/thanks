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

    create_dir(root_dir)?;
    copy_public_assets(root_dir)?;

    let hb = hb()?;
    render_about_page(&hb, root_dir)?;

    let mut combined_all_time = AuthorMap::new();
    for data in projects {
        let project_out_dir = root_dir.join(data.project.url_path());
        create_dir(&project_out_dir)?;

        if data.project.is_versionless() {
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
            backlink: "all projects",
        },
    )?;

    fs::write(root_dir.join("all-time.html"), res)?;

    render_projects_page(
        &hb,
        projects,
        &combined_all_time,
        &root_dir.join("index.html"),
    )?;

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
            name: data.project.name(),
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
    path: &Path,
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
            name: data.project.name().to_string(),
            link: data.project.url_path().to_string(),
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

    fs::write(path, res)?;
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
    backlink: &'static str,
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
            common: CommonData::new(format!("All-time {} Contributors", data.project.name())),
            release_title: String::from("All-time"),
            release: data.project.name().to_string(),
            count: scores.len(),
            scores,
            in_progress: true,
            backlink: if data.project.is_versionless() {
                "all projects"
            } else {
                "all releases"
            },
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
                common: CommonData::new(format!("{} {version} Contributors", data.project.name())),
                release_title: version.name.clone(),
                release: version.to_string(),
                count: scores.len(),
                scores,
                in_progress: version.in_progress,
                backlink: "all releases",
            },
        )?;

        let version_dir = output_dir.join(version.to_string());
        create_dir(&version_dir)?;
        fs::write(version_dir.join("index.html"), res)?;
    }
    Ok(())
}
