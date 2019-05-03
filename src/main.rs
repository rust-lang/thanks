#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use hyperx::header::Header;
use serde::Deserialize;
use std::env;

#[derive(Clone)]
struct Client {
    http: reqwest::Client,
}

impl Client {
    fn new() -> Client {
        Client {
            http: reqwest::Client::new(),
        }
    }

    fn github(&self) -> GithubClient<'_> {
        GithubClient { client: self }
    }
}

struct GithubClient<'a> {
    client: &'a Client,
}

impl<'a> GithubClient<'a> {
    fn commits(&self, repo_slug: &str, until: &str, stop_at: Option<&str>) -> Vec<models::Commit> {
        fn deserialize_request(resp: &mut reqwest::Response) -> Vec<models::Commit> {
            #[derive(Deserialize)]
            struct ApiResponse {
                commit: ApiCommit,
                sha: String,
            }

            #[derive(Deserialize)]
            struct ApiCommit {
                author: ApiPerson,
            }

            #[derive(Deserialize)]
            struct ApiPerson {
                name: String,
                email: String,
            }

            let v: Vec<ApiResponse> = resp.json().unwrap();

            v.into_iter()
                .map(|ApiResponse { commit: c, sha }| models::Commit {
                    sha,
                    author: c.author.name,
                    email: c.author.email,
                })
                .collect()
        }

        let mut commits: Vec<models::Commit> = Vec::new();
        let mut next_url = format!(
            "https://api.github.com/repos/{}/commits?sha={}&per_page=100",
            repo_slug, until,
        );
        loop {
            // check if commits contains stop commit and truncate at it
            if let Some(stop) = stop_at {
                if let Some(idx) = commits.iter().rposition(|c| c.sha == stop) {
                    commits.truncate(idx);
                    break;
                }
            }

            eprintln!("get: {:?}, {} commits so far", next_url, commits.len());
            let req = self.client.http.get(&next_url).header(
                "Authorization",
                format!("token {}", env::var("GH_TOKEN").unwrap()),
            );
            let mut resp = match req.send() {
                Ok(v) => v,
                Err(err) => {
                    panic!("failed to get commits with err: {:?}", err);
                }
            };
            commits.extend(deserialize_request(&mut resp));

            if let Some(link) = resp.headers().get(reqwest::header::LINK) {
                let link = hyperx::header::Link::parse_header(&link).unwrap();
                if let Some(next) = next_link(&link) {
                    next_url = next.to_owned();
                    continue;
                }
            }

            break;
        }

        commits
    }
}

fn next_link(link: &hyperx::header::Link) -> Option<&str> {
    link.values()
        .iter()
        .find(|lv| lv.rel() == Some(&[hyperx::header::RelationType::Next]))
        .map(|lv| lv.link())
}

pub mod models;
pub mod schema;

fn main() {
    dotenv::dotenv().ok();
    let client = Client::new();

    handle_repo(&client, "rust-lang/rls");
    handle_repo(&client, "rust-lang/cargo");
    handle_repo(&client, "rust-lang/rust");
}

fn handle_repo(client: &Client, slug: &str) {
    let db_url = std::env::var("DATABASE_URL").expect("`DATABASE_URL` must be set");
    let conn =
        SqliteConnection::establish(&db_url).expect(&format!("Error connecting to `{}`", db_url));
    let repo: Option<models::Repository> = schema::repositories::table
        .find(slug)
        .get_result(&conn)
        .optional()
        .unwrap();

    let repo = match repo {
        Some(r) => r,
        None => {
            let default = models::Repository {
                name: String::from(slug),
                latest_commit: None,
            };
            diesel::insert_into(schema::repositories::table)
                .values(&default)
                .execute(&conn)
                .unwrap();
            default
        }
    };

    let commits = client.github().commits(
        slug,
        "master",
        repo.latest_commit.as_ref().map(|s| s.as_str()),
    );

    if !commits.is_empty() {
        diesel::update(schema::repositories::table.filter(schema::repositories::name.eq(slug)))
            .set(schema::repositories::latest_commit.eq(commits.first().map(|c| c.sha.as_str())))
            .execute(&conn)
            .unwrap();
    }

    diesel::insert_or_ignore_into(schema::commits::dsl::commits)
        .values(&commits)
        .execute(&conn)
        .expect("insert success");
}
