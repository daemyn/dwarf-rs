use chrono::Utc;
use sqlx::{Error, Pool, Postgres};

use crate::{models::DwarfUrl, utils::generate_slug};

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
    // Recursive fn is an option here but it requires Pin and make the code hard to read
    loop {
        let now = Utc::now();
        let slug = generate_slug(slug_size);

        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) 
            FROM dwarf_urls 
            WHERE slug = $1
            "#,
        )
        .bind(&slug)
        .fetch_one(pool)
        .await?;

        if count > 0 {
            continue;
        }

        let dwarf_url = sqlx::query_as!(
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
        .await?;

        return Ok(dwarf_url);
    }
}
