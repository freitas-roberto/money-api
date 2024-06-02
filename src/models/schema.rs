// @generated automatically by Diesel CLI.

diesel::table! {
    agencies (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
        bank_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    banks (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(agencies -> banks (bank_id));

diesel::allow_tables_to_appear_in_same_query!(
    agencies,
    banks,
);
