use crate::db::{url::Url, DbConnection};

const SERVICE_FN_TYPE: &str = "SERVICE";

#[tracing::instrument(
    name = "short-url",
    skip(pool),
    fields(fn_type = %SERVICE_FN_TYPE, url = %url)
)]
pub fn short_url(pool: &mut DbConnection, url: String) -> Option<Url> {
    let Some(random_id) = generate_random_id(pool, None) else {
        tracing::error!("Error while  generating random id.");
        return None;
    };

    let hash = murmurhash32::murmurhash3(random_id.as_bytes()) as i64;

    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::days(1);

    let url = Url {
        original_url: url,
        short_url: random_id,
        created_on: now.timestamp(),
        expiries_at: expires_at.timestamp(),
        redirection_count: 0,
        id: 0,
        hash
    };

    Url::save_url(url, pool)
}

fn generate_random_id(pool: &mut DbConnection, retry: Option<u8>) -> Option<String> {
    let id = nanoid::nanoid!(6);
    let retry = retry.unwrap_or(0);

    if retry == 10 {
        tracing::error!("Retry limit exceeded for generating random id.");
        return None;
    }

    if let Some(_) = Url::get_url(pool, &id) {
        let retry = retry + 1;
        tracing::warn!("Retrying to generate random id. Times {:?}", retry);

        return generate_random_id(pool, Some(retry));
    } else {
        Some(id)
    }
}
