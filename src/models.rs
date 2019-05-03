use crate::schema::{commits, repositories};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "commits"]
pub struct Commit {
    pub sha: String,
    pub author: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "commits"]
pub struct NewCommit<'a> {
    pub sha: &'a str,
    pub author: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "repositories"]
pub struct Repository {
    // slug
    pub name: String,
    pub latest_commit: Option<String>,
}
