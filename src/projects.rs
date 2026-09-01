use crate::git::{VersionTag, get_versions};
use git2::Repository;
use semver::Version;

pub trait Project {
    /// Name of the project, displayed on the website.
    const NAME: &'static str;

    /// Path under which the project will be available on the web.
    /// If the `url` is e.g. `rust`, it will be available under `/rust/`.
    const URL_PATH: &'static str;

    /// Should this project be displayed as the main homepage project?
    const IS_HOMEPAGE: bool = false;

    /// URL of its GitHub repository.
    const REPO_URL: &'static str;

    /// Identify the versions that have been tagged in the given repo, including
    /// any project-specific additional versions to add.
    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>>;

    /// true if the project does not track versions explicitly.
    /// It will be rendered as a single page with all-time contributions.
    const IS_VERSIONLESS: bool = false;

    /// Contributions from users with these e-mail addresses will be ignored.
    /// The addresses will be compared in a case-insensitive manner.
    const IGNORED_EMAILS: &'static [&'static str] = &[];
}

pub struct Rust;

impl Project for Rust {
    const NAME: &'static str = "Rust";
    const URL_PATH: &'static str = "rust";
    const IS_HOMEPAGE: bool = true;
    const REPO_URL: &'static str = "https://github.com/rust-lang/rust.git";

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        let mut versions = get_versions(repo, Self::NAME)?;
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

        Ok(versions)
    }
}

pub struct Rustup;

impl Project for Rustup {
    const NAME: &'static str = "Rustup";
    const URL_PATH: &'static str = "rustup";
    const REPO_URL: &'static str = "https://github.com/rust-lang/rustup.git";

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        let mut versions = get_versions(repo, Self::NAME)?;
        let last_full_stable = versions
            .iter()
            .rfind(|v| v.raw_tag.ends_with(".0"))
            .unwrap()
            .version
            .clone();

        versions.push(VersionTag {
            name: String::from("Nightly"),
            version: {
                let mut last = last_full_stable.clone();
                last.minor += 1;
                last
            },
            raw_tag: String::from("main"),
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        });

        Ok(versions)
    }
}

pub struct CratesIo;

impl Project for CratesIo {
    const NAME: &'static str = "crates.io";
    const URL_PATH: &'static str = "crates.io";
    const IS_VERSIONLESS: bool = true;
    const REPO_URL: &'static str = "https://github.com/rust-lang/crates.io.git";

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        Ok(vec![VersionTag {
            name: String::from("Nightly"),
            version: Version::new(1, 0, 0),
            raw_tag: String::from("main"),
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        }])
    }
}

pub struct DocsRs;

impl Project for DocsRs {
    const NAME: &'static str = "Docs.rs";
    const URL_PATH: &'static str = "docs.rs";
    const IS_VERSIONLESS: bool = true;
    const REPO_URL: &'static str = "https://github.com/rust-lang/docs.rs.git";
    const IGNORED_EMAILS: &'static [&'static str] = &[
        // CI bot
        "docs.rs@users.noreply.github.com",
        // Renovatebot
        "29139614+renovate[bot]@users.noreply.github.com",
        // Dependabot
        "49699333+dependabot[bot]@users.noreply.github.com",
    ];

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        Ok(vec![VersionTag {
            name: String::from("Nightly"),
            version: Version::new(1, 0, 0),
            raw_tag: String::from("main"),
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        }])
    }
}
