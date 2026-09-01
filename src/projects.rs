use crate::git::{VersionTag, get_versions};
use git2::Repository;

pub trait Project {
    /// Name of the project, displayed on the website.
    fn name(&self) -> &'static str;

    /// Path under which the project will be available on the web.
    /// If the `url` is e.g. `rust`, it will be available under `/rust/`.
    fn url_path(&self) -> &'static str;

    /// Should this project be displayed as the main homepage project?
    fn is_homepage(&self) -> bool {
        false
    }

    /// URL of its GitHub repository.
    fn repo_url(&self) -> &'static str;

    /// Identify the versions that have been tagged in the given repo, including
    /// any project-specific additional versions to add.
    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>>;

    /// Returns true if the project does not track versions explicitly.
    /// It will be rendered as a single page with all-time contributions.
    fn is_versionless(&self) -> bool {
        false
    }
}

pub struct Rust;

impl Project for Rust {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn url_path(&self) -> &'static str {
        "rust"
    }

    fn is_homepage(&self) -> bool {
        true
    }

    fn repo_url(&self) -> &'static str {
        "https://github.com/rust-lang/rust.git"
    }

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        let mut versions = get_versions(repo, self.name())?;
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
    fn name(&self) -> &'static str {
        "Rustup"
    }

    fn url_path(&self) -> &'static str {
        "rustup"
    }

    fn repo_url(&self) -> &'static str {
        "https://github.com/rust-lang/rustup.git"
    }

    fn get_versions(
        &self,
        repo: &Repository,
    ) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        let mut versions = get_versions(repo, self.name())?;
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
