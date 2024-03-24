use rocket::futures::StreamExt as _;
use serde::Serialize;
use sqlx::{
    types::chrono::{NaiveDate, NaiveDateTime},
    Executor as _, MySqlPool, Row as _,
};
use std::net::IpAddr;

use super::error::Error;

#[derive(Debug)]
pub struct Player {
    pub ckey: String,
    pub byond_key: String,
    pub first_seen: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub first_seen_round: u32,
    pub last_seen_round: u32,
    pub ip: IpAddr,
    pub cid: String,
    pub byond_age: NaiveDate,
}

pub async fn get_player(ckey: &str, pool: &MySqlPool) -> Result<Player, Error> {
    let mut connection = pool.acquire().await?;

    let query = sqlx::query("SELECT ckey, byond_key, firstseen, firstseen_round_id, lastseen, lastseen_round_id, INET_NTOA(ip), computerid, accountjoindate FROM player WHERE LOWER(ckey) = ?");
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
        ip: row.try_get("INET_NTOA(ip)")?,
        cid: row.try_get("computerid")?,
        byond_age: row.try_get("accountjoindate")?,
    };

    connection.close().await?;

    Ok(player)
}

#[derive(Debug, Serialize)]
pub struct Roletime {
    ckey: String,
    minutes: u32,
}

pub async fn get_top_roletime(job: &str, pool: &MySqlPool) -> Result<Vec<Roletime>, Error> {
    let mut connection = pool.acquire().await?;

    let query = sqlx::query(
        "SELECT ckey, minutes FROM role_time WHERE LOWER(job) = ? ORDER BY minutes DESC LIMIT 15",
    );
    let bound = query.bind(job.to_lowercase());

    let mut roletimes = Vec::new();

    {
        let mut rows = connection.fetch(bound);

        while let Some(row) = rows.next().await {
            let row = row?;

            let roletime = Roletime {
                ckey: row.try_get("ckey")?,
                minutes: row.try_get("minutes")?,
            };

            roletimes.push(roletime);
        }
    }

    connection.close().await?;

    Ok(roletimes)
}
