// @generated automatically by Diesel CLI.

diesel::table! {
    url (id) {
        id -> Int4,
        #[max_length = 255]
        original_url -> Varchar,
        #[max_length = 255]
        short_url -> Varchar,
        created_on -> Int8,
        expiries_at -> Int8,
        redirection_count -> Int4,
        hash -> Int8,
    }
}
