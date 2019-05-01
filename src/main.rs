use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

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

impl<'a> GithubClient<'a> {}

fn main() {
    dotenv::dotenv().ok();
    let client = Client::new();
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set");
    let conn =
        SqliteConnection::establish(&db_url).expect(&format!("Error connecting to `{}`", db_url));
}
