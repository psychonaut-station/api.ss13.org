use std::collections::HashSet;

use chrono::NaiveDateTime;
use rocket::futures::StreamExt as _;
use serde::Serialize;
use sqlx::{Executor as _, MySqlPool, Row as _};

use super::error::Error;

#[derive(Debug, Clone, Serialize)]
pub struct TestMerge {
    round_id: u32,
    #[serde(with = "crate::serde::datetime")]
    datetime: NaiveDateTime,
    test_merges: Vec<u32>,
}

pub async fn get_recent_test_merges(pool: &MySqlPool) -> Result<Vec<TestMerge>, Error> {
    let mut connection = pool.acquire().await?;

    let query = sqlx::query(
        "SELECT round_id, datetime, JSON_EXTRACT(json, '$.data.*.number') AS test_merges FROM tg.feedback WHERE key_name = 'testmerged_prs' ORDER BY round_id DESC LIMIT 200"
    );

    let mut recent_test_merges = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let row = row?;

            let test_merges = row.try_get::<String, _>("test_merges")?;

            let test_merges = serde_json::from_str::<HashSet<String>>(&test_merges)?
                .into_iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;

            let test_merge = TestMerge {
                round_id: row.try_get("round_id")?,
                datetime: row.try_get("datetime")?,
                test_merges,
            };

            recent_test_merges.push(test_merge);
        }
    }

    connection.close().await?;

    Ok(recent_test_merges)
}
