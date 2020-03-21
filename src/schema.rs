table! {
    languages (id) {
        id -> Integer,
        language -> Text,
    }
}

table! {
    words (id) {
        id -> Integer,
        word -> Text,
        language_id -> Integer,
    }
}

joinable!(words -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    languages,
    words,
);
