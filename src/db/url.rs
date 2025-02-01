use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::url)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    id: i32,
    original_url: String,
    short_url: String,
    created_on: i64,
    expiries_at: i64,
    redirection_count: i32,
}
