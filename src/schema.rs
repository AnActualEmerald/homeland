table! {
    posts (id) {
        id -> Int8,
        author -> Varchar,
        title -> Varchar,
        body -> Text,
        published -> Nullable<Bool>,
        timestamp -> Timestamp,
    }
}

table! {
    projects (title) {
        title -> Varchar,
        short_desc -> Varchar,
        long_desc -> Text,
        repo -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    projects,
    users,
);
