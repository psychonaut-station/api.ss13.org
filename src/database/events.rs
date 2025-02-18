use chrono::NaiveDateTime;
use rocket::futures::StreamExt as _;
use serde::Serialize;
use sqlx::{Executor as _, MySqlPool, Row as _};

use super::error::Error;

#[derive(Debug, Serialize)]
pub struct Death {
    pub name: String,
    pub job: String,
    pub pod: String,
    pub brute: u32,
    pub fire: u32,
    pub oxy: u32,
    pub tox: u32,
    pub last_words: Option<String>,
    pub suicide: bool,
    pub round_id: u32,
    #[serde(with = "crate::serde::datetime")]
    pub timestamp: NaiveDateTime,
}

pub async fn get_deaths(since: Option<&str>, pool: &MySqlPool) -> Result<Vec<Death>, Error> {
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT name, job, pod, brute, fire, oxy, tox, last_words, suicide, round_id, tod AS timestamp FROM death".to_string();

    if since.is_some() {
        sql.push_str(" WHERE timestamp > ?");
    }

    sql.push_str(" GROUP BY timestamp");

    let mut query = sqlx::query(&sql);

    if let Some(since) = since {
        query = query.bind(since);
    }

    let mut deaths = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let death = row?;

            let death = Death {
                name: death.try_get("name")?,
                job: death.try_get("job")?,
                pod: death.try_get("pod")?,
                brute: death.try_get("brute")?,
                fire: death.try_get("fire")?,
                oxy: death.try_get("oxy")?,
                tox: death.try_get("tox")?,
                last_words: death.try_get("last_words")?,
                suicide: death.try_get("suicide")?,
                round_id: death.try_get("round_id")?,
                timestamp: death.try_get("timestamp")?,
            };

            deaths.push(death);
        }
    }

    connection.close().await?;
    Ok(deaths)
}

#[derive(Debug, Serialize)]
pub struct Citation {
    pub sender: String,
    pub recipient: String,
    pub message: String,
    pub fine: u32,
    pub round_id: u32,
    #[serde(with = "crate::serde::datetime")]
    pub timestamp: NaiveDateTime,
}

pub async fn get_citations(since: Option<&str>, pool: &MySqlPool) -> Result<Vec<Citation>, Error> {
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT round_id, sender_ic, recipient, message, fine, timestamp FROM citation".to_string();

    if since.is_some() {
        sql.push_str(" WHERE timestamp > ?");
    }

    sql.push_str(" GROUP BY timestamp");

    let mut query = sqlx::query(&sql);

    if let Some(since) = since {
        query = query.bind(since);
    }

    let mut citations = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let citation = row?;

            let citation = Citation {
                round_id: citation.try_get("round_id")?,
                sender: citation.try_get("sender_ic")?,
                recipient: citation.try_get("recipient")?,
                message: citation.try_get("message")?,
                fine: citation.try_get("fine")?,
                timestamp: citation.try_get("timestamp")?,
            };

            citations.push(citation);
        }
    }

    connection.close().await?;
    Ok(citations)
}
