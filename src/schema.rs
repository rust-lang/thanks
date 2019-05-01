table! {
    commits (sha) {
        sha -> Text,
        author -> Text,
        email -> Text,
    }
}

table! {
    repositories (name) {
        name -> Text,
        latest_commit -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    commits,
    repositories,
);
