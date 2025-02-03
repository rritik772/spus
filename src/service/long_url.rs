use crate::db::{url::Url, DbConnection};

const FN_TYPE: &str = "LONG_URL";

#[tracing::instrument(name="long-url", skip(pool), fields(url=%url, fn_type=%FN_TYPE))]
pub fn long_url(pool: &mut DbConnection, url: String) -> Option<Url> {
    let hash = murmurhash32::murmurhash3(url.as_bytes());
    let Some(urls) = Url::get_url_by_hash(pool, hash as i64) else {
        return None;
    };

    urls.iter().filter(|v| v.short_url == url).cloned().next()
}
