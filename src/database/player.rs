use sqlx::{
    types::chrono::{NaiveDate, NaiveDateTime},
    Executor as _, MySqlPool, Row as _,
};

use super::error::Error;

#[derive(Debug)]
pub struct Player {
    pub ckey: String,
    pub byond_key: String,
    pub first_seen: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub first_seen_round: u32,
    pub last_seen_round: u32,
    pub cid: String,
    pub byond_age: NaiveDate,
}

pub async fn get_player(ckey: &str, pool: &MySqlPool) -> Result<Player, Error> {
    let mut connection = pool.acquire().await?;

    let query = sqlx::query("SELECT ckey, byond_key, firstseen, firstseen_round_id, lastseen, lastseen_round_id, computerid, accountjoindate FROM player WHERE LOWER(ckey) = ?");
    let bound = query.bind(ckey.to_lowercase());

    let Ok(row) = connection.fetch_one(bound).await else {
        return Err(Error::PlayerNotFound);
    };

    let player = Player {
        ckey: row.try_get("ckey")?,
        byond_key: row.try_get("byond_key")?,
        first_seen: row.try_get("firstseen")?,
        last_seen: row.try_get("lastseen")?,
        first_seen_round: row.try_get("firstseen_round_id")?,
        last_seen_round: row.try_get("lastseen_round_id")?,
        cid: row.try_get("computerid")?,
        byond_age: row.try_get("accountjoindate")?,
    };

    connection.close().await?;

    Ok(player)
}
