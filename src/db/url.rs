use serde::{Serialize, Deserialize};
use crate::db::DbConnection;
use crate::schema::url::dsl::*;

use diesel::{insert_into, prelude::*};

const URL_FN_TYPE: &str = "URL";

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::url)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_url: String,
    pub created_on: i64,
    pub expiries_at: i64,
    pub redirection_count: i32,
    pub hash: i64,
}


impl Url {
    #[tracing::instrument(
        name = "get-url",
        skip(conn),
        fields(
            short_url = %_short_url,
            fn_type = %URL_FN_TYPE
        )
    )]
    pub fn get_url(conn: &mut DbConnection, _short_url: &str) -> Option<Self> {
        let urls = url
            .filter(short_url.eq(_short_url))
            .limit(1)
            .select(Url::as_select())
            .load(conn);

        match urls {
            Ok(v) => v.into_iter().next(),
            Err(e) => {
                tracing::error!("Error while getting url with short_url: {}, E: {:?}", _short_url, e);
                None
            }
        }
    }

    #[tracing::instrument(
        name = "get-url",
        skip(conn),
        fields(
            short_url = %_hash,
            fn_type = %URL_FN_TYPE
        )
    )]
    pub fn get_url_by_hash(conn: &mut DbConnection, _hash: i64) -> Option<Self> {
        let urls = url
            .filter(hash.eq(_hash))
            .limit(1)
            .select(Url::as_select())
            .load(conn);

        match urls {
            Ok(v) => v.into_iter().next(),
            Err(e) => {
                tracing::error!("Error while getting url with short_url: {}, E: {:?}", _hash, e);
                None
            }
        }
    }

    #[tracing::instrument(
        name = "save-url",
        skip(conn),
        fields(fn_type = %URL_FN_TYPE)
    )]
    pub fn save_url(self, conn: &mut DbConnection) -> Option<Self> {
        let url_value = (
            original_url.eq(&self.original_url),
            short_url.eq(&self.short_url),
            created_on.eq(&self.created_on),
            expiries_at.eq(&self.expiries_at),
            redirection_count.eq(&self.redirection_count),
            hash.eq(&self.hash)
        );

        let urls = insert_into(url)
            .values(url_value)
            .get_results(conn);

        match urls {
            Ok(v) => v.into_iter().next(),
            Err(e) => {
                tracing::error!("Cannot insert Url Object. {:?} E: {:?}", &self, e);
                None
            }
        }
    }

}
