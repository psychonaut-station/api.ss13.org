use std::collections::HashSet;

use rocket::futures::StreamExt as _;
use serde::{ser::SerializeStruct as _, Serialize, Serializer};
use sqlx::{types::chrono::NaiveDateTime, Executor as _, MySqlPool, Row as _};

use super::error::Error;

#[derive(Debug, Clone)]
pub struct TestMerge {
    round_id: u32,
    datetime: NaiveDateTime,
    test_merges: Vec<u32>,
}

impl Serialize for TestMerge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TestMerge", 3)?;
        state.serialize_field("round_id", &self.round_id)?;
        state.serialize_field("datetime", &self.datetime.to_string())?;
        state.serialize_field("test_merges", &self.test_merges)?;
        state.end()
    }
}

pub async fn get_recent_test_merges(pool: &MySqlPool) -> Result<Vec<TestMerge>, Error> {
    let mut connection = pool.acquire().await?;

    let query = sqlx::query(
        "SELECT round_id, datetime, JSON_EXTRACT(json, '$.data.*.number') AS test_merges FROM tg.feedback WHERE key_name = 'testmerged_prs' ORDER BY round_id DESC LIMIT 200"
    );

    let mut test_merges = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let row = row?;

            let merges = serde_json::from_str::<HashSet<String>>(
                row.try_get::<String, _>("test_merges")?.as_str(),
            )?
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

            let test_merge = TestMerge {
                round_id: row.try_get("round_id")?,
                datetime: row.try_get("datetime")?,
                test_merges: merges,
            };

            test_merges.push(test_merge);
        }
    }

    connection.close().await?;

    Ok(test_merges)
}
