use chrono::Utc;
use sqlx::{Error, Pool, Postgres};
use log::{error, warn};
use crate::{models::DwarfUrl, utils::generate_slug};

const MAX_ATTEMPTS: u8 = 10;

pub async fn visit_url(pool: &Pool<Postgres>, slug: &str) -> Result<DwarfUrl, Error> {
    let dwarf_url = sqlx::query_as!(
        DwarfUrl,
        r#"
        UPDATE dwarf_urls
        SET visit_count = visit_count + 1, updated_at = NOW()
        WHERE slug = $1
        RETURNING *
        "#,
        slug
    )
    .fetch_one(pool)
    .await?;

    Ok(dwarf_url)
}

pub async fn generate_url(
    pool: &Pool<Postgres>,
    target: &str,
    slug_size: u8,
) -> Result<DwarfUrl, Error> {
    let mut attempts: u8 = 0;
    loop {
        attempts += 1;

        if attempts > MAX_ATTEMPTS {
            error!( "Max attempts reached while generating URL. Attempts: {}", attempts);
            return Err(Error::RowNotFound);
        }

        let now = Utc::now();
        let slug = generate_slug(slug_size);

        match sqlx::query_as!(
            DwarfUrl,
            r#"
            INSERT INTO dwarf_urls (slug, target, visit_count, created_at, updated_at)
            VALUES ($1, $2, 0, $3, $3)
            RETURNING *
            "#,
            slug,
            target,
            now,
        )
        .fetch_one(pool)
        .await
        {
            Ok(dwarf_url) => {
                return Ok(dwarf_url);
            }
            Err(sqlx::Error::Database(err)) if err.code().unwrap_or_default() == "23505" => {
                warn!(
                    "Slug collision detected for: '{}'. Retrying... Attempt: {}",
                    slug, attempts
                );
                continue;
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
