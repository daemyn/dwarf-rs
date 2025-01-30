use crate::{errors::AppError, models::DwarfUrl, utils::generate_slug};
use chrono::Utc;
use log::{error, warn};
use sqlx::{Pool, Postgres};

const MAX_ATTEMPTS: u8 = 10;
const BLACK_LIST_WORDS: [&str; 1] = ["health"];

pub async fn service_health_check(pool: &Pool<Postgres>) -> Result<(), AppError> {
    match sqlx::query_scalar!("SELECT 1").fetch_one(pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::ServiceUnavailable),
    }
}

pub async fn get_url_by_slug(pool: &Pool<Postgres>, slug: &str) -> Result<DwarfUrl, AppError> {
    match sqlx::query_as!(
        DwarfUrl,
        r#"
        SELECT * FROM dwarf_urls
        WHERE slug = $1
        "#,
        slug
    )
    .fetch_one(pool)
    .await
    {
        Ok(dwarf_url) => Ok(dwarf_url),
        Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalError),
    }
}

pub async fn visit_url(pool: &Pool<Postgres>, slug: &str) -> Result<DwarfUrl, AppError> {
    match sqlx::query_as!(
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
    .await
    {
        Ok(dwarf_url) => Ok(dwarf_url),
        Err(sqlx::Error::RowNotFound) => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalError),
    }
}

pub async fn generate_url(
    pool: &Pool<Postgres>,
    target: &str,
    slug_size: u8,
) -> Result<DwarfUrl, AppError> {
    let mut attempts: u8 = 0;
    loop {
        attempts += 1;

        if attempts > MAX_ATTEMPTS {
            error!(
                "Max attempts reached while generating URL. Attempts: {}",
                attempts
            );
            return Err(AppError::MaxAttemptsReached);
        }

        let now = Utc::now();
        let slug = generate_slug(slug_size);

        if BLACK_LIST_WORDS.contains(&slug.as_str()) {
            continue;
        }

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
            Ok(dwarf_url) => return Ok(dwarf_url),
            Err(sqlx::Error::Database(err)) if err.code().unwrap_or_default() == "23505" => {
                warn!(
                    "Slug collision detected for: '{}'. Retrying... Attempt: {}",
                    slug, attempts
                );
                continue;
            }
            Err(_) => return Err(AppError::InternalError),
        }
    }
}
