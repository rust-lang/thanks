use crate::mailmap::Author;
use std::collections::HashMap;

pub struct Reviewers(HashMap<&'static str, Author>);

impl Reviewers {
    pub fn new() -> Self {
        let map = HashMap::new();
        Reviewers(map)
    }

    pub fn to_author(&self, reviewer: &str) -> Author {
        self.0.get(reviewer).cloned().unwrap_or_else(|| Author {
            name: reviewer.into(),
            email: String::new(),
        })
    }
}
