use crate::AuthorMap;
use std::collections::HashMap;
use unicase::UniCase;

#[derive(serde::Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuthorScore {
    pub rank: u32,
    pub author: String,
    pub email: String,
    pub commits: usize,
}

pub fn author_map_to_scores(map: &AuthorMap) -> Vec<AuthorScore> {
    let debug_emails = std::env::var("DEBUG_EMAILS").is_ok_and(|value| value == "1");

    let scores = map
        .iter()
        .map(|(author, commits)| {
            let name = UniCase::into_inner(author.name.clone());

            AuthorScore {
                rank: 0,
                author: if debug_emails {
                    format!("{name} ({})", UniCase::into_inner(author.email.clone()))
                } else {
                    name
                },
                email: UniCase::into_inner(author.email.clone()),
                commits,
            }
        })
        .collect::<Vec<_>>();
    let mut scores = deduplicate_scores(scores);
    scores.sort_by_key(|e| (std::cmp::Reverse(e.commits), e.author.clone()));

    let mut last_rank = 1;
    let mut ranked_at_current = 0;
    let mut last_commits = usize::MAX;
    for entry in &mut scores {
        if entry.commits < last_commits {
            last_commits = entry.commits;
            last_rank += ranked_at_current;
            ranked_at_current = 1;
        } else {
            ranked_at_current += 1;
        }
        entry.rank = last_rank;
    }
    scores
}

/// Deduplicate scores based on the assumption that an e-mail uniquely identifies a given
/// person. If there are multiple entries with the same email, their commit counts will be
/// merged into a single entry, with the canonical name being chosen based on the entry with
/// the most commits.
fn deduplicate_scores(entries: Vec<AuthorScore>) -> Vec<AuthorScore> {
    let mut entry_map: HashMap<String, Vec<AuthorScore>> = HashMap::with_capacity(entries.len());
    for entry in entries {
        entry_map
            .entry(entry.email.clone())
            .or_default()
            .push(entry);
    }

    entry_map
        .into_values()
        .map(|mut entry| {
            // If there are multiple entries with the same maximum commit count, ensure that
            // the ordering is stable, by sorting based on the whole entry.
            entry.sort();
            let canonical_entry = entry.iter().max_by_key(|entry| entry.commits).unwrap();
            AuthorScore {
                rank: 0,
                author: canonical_entry.author.clone(),
                email: canonical_entry.email.clone(),
                commits: entry.iter().map(|e| e.commits).sum(),
            }
        })
        .collect()
}
