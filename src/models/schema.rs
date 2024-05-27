diesel::table! {
    banks (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}