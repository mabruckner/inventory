table! {
    batches (id) {
        id -> Integer,
        class -> Integer,
        quantity -> Text,
        data -> Text,
    }
}

table! {
    classes (id) {
        id -> Integer,
        name -> Text,
        unit -> Text,
        schema -> Text,
    }
}

table! {
    items (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(batches -> classes (class));

allow_tables_to_appear_in_same_query!(
    batches,
    classes,
    items,
);
