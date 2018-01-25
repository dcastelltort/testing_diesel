table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        publish_at -> Timestamp,
        visit_count -> Int4,
    }
}