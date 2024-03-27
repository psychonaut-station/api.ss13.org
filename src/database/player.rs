use rocket::futures::StreamExt as _;
use serde::{ser::SerializeStruct as _, Serialize, Serializer};
use sqlx::{
    query,
    types::chrono::{NaiveDate, NaiveDateTime},
    Executor as _, MySqlPool, Row as _,
};
use std::net::IpAddr;

use super::error::Error;

#[derive(Debug)]
pub struct Player {
    pub ckey: String,
    pub byond_key: Option<String>,
    pub first_seen: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub first_seen_round: Option<u32>,
    pub last_seen_round: Option<u32>,
    pub ip: IpAddr,
    pub cid: String,
    pub byond_age: Option<NaiveDate>,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Player", 9)?;
        state.serialize_field("ckey", &self.ckey)?;
        state.serialize_field("byond_key", &self.byond_key)?;
        state.serialize_field("first_seen", &self.first_seen.to_string())?;
        state.serialize_field("last_seen", &self.last_seen.to_string())?;
        state.serialize_field("first_seen_round", &self.first_seen_round)?;
        state.serialize_field("last_seen_round", &self.last_seen_round)?;
        state.serialize_field("ip", &self.ip.to_string())?;
        state.serialize_field("cid", &self.cid)?;
        state.serialize_field(
            "byond_age",
            &self.byond_age.as_ref().map(ToString::to_string),
        )?;
        state.end()
    }
}

pub async fn get_player(ckey: &str, pool: &MySqlPool) -> Result<Player, Error> {
    let mut connection = pool.acquire().await?;

    let query = query("SELECT ckey, byond_key, firstseen, firstseen_round_id, lastseen, lastseen_round_id, INET_NTOA(ip), computerid, accountjoindate FROM player WHERE LOWER(ckey) = ?");
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
pub struct JobRoletime {
    ckey: String,
    minutes: u32,
}

pub async fn get_top_roletime(job: &str, pool: &MySqlPool) -> Result<Vec<JobRoletime>, Error> {
    let mut connection = pool.acquire().await?;

    let query = query(
        "SELECT ckey, minutes FROM role_time WHERE LOWER(job) = ? ORDER BY minutes DESC LIMIT 15",
    );
    let bound = query.bind(job.to_lowercase());

    let mut roletimes = Vec::new();

    {
        let mut rows = connection.fetch(bound);

        while let Some(row) = rows.next().await {
            let row = row?;

            let roletime = JobRoletime {
                ckey: row.try_get("ckey")?,
                minutes: row.try_get("minutes")?,
            };

            roletimes.push(roletime);
        }
    }

    connection.close().await?;

    Ok(roletimes)
}

#[derive(Debug, Serialize)]
pub struct PlayerRoletime {
    job: String,
    minutes: u32,
}

pub async fn get_roletime(ckey: &str, pool: &MySqlPool) -> Result<Vec<PlayerRoletime>, Error> {
    let mut connection = pool.acquire().await?;

    let query =
        query("SELECT job, minutes FROM role_time WHERE LOWER(ckey) = ? ORDER BY minutes DESC");
    let bound = query.bind(ckey.to_lowercase());

    let mut roletimes = Vec::new();

    {
        let mut rows = connection.fetch(bound);

        while let Some(row) = rows.next().await {
            let row = row?;

            let roletime = PlayerRoletime {
                job: row.try_get("job")?,
                minutes: row.try_get("minutes")?,
            };

            roletimes.push(roletime);
        }
    }

    connection.close().await?;

    Ok(roletimes)
}

pub async fn get_jobs(pool: &MySqlPool) -> Result<Vec<String>, Error> {
    let mut connection = pool.acquire().await?;

    let query = query("SELECT DISTINCT job FROM role_time ORDER BY job ASC");

    let mut jobs = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let row = row?;

            let job = row.try_get("job")?;

            jobs.push(job);
        }
    }

    connection.close().await?;

    Ok(jobs)
}

pub async fn get_ckey(ckey: &str, pool: &MySqlPool) -> Result<Vec<String>, Error> {
    let mut connection = pool.acquire().await?;

    let query = query("SELECT ckey FROM player WHERE ckey LIKE ? ORDER BY ckey LIMIT 25");
    let bound = query.bind(format!("{ckey}%"));

    let mut ckeys = Vec::new();

    {
        let mut rows = connection.fetch(bound);

        while let Some(row) = rows.next().await {
            let row = row?;

            let ckey = row.try_get("ckey")?;

            ckeys.push(ckey);
        }
    }

    connection.close().await?;

    Ok(ckeys)
}

#[derive(Debug)]
pub struct Ban {
    pub id: u32,
    pub bantime: NaiveDateTime,
    pub round_id: Option<u32>,
    pub role: Option<String>,
    pub expiration_time: Option<NaiveDateTime>,
    pub reason: String,
    pub ckey: Option<String>,
    pub a_ckey: String,
    pub edits: Option<String>,
    pub unbanned_datetime: Option<NaiveDateTime>,
    pub unbanned_ckey: Option<String>,
}

impl Serialize for Ban {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Ban", 11)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("bantime", &self.bantime.to_string())?;
        state.serialize_field("round_id", &self.round_id)?;
        state.serialize_field("role", &self.role)?;
        state.serialize_field(
            "expiration_time",
            &self.expiration_time.as_ref().map(ToString::to_string),
        )?;
        state.serialize_field("reason", &self.reason)?;
        state.serialize_field("ckey", &self.ckey)?;
        state.serialize_field("a_ckey", &self.a_ckey)?;
        state.serialize_field("edits", &self.edits)?;
        state.serialize_field(
            "unbanned_datetime",
            &self.unbanned_datetime.as_ref().map(ToString::to_string),
        )?;
        state.serialize_field("unbanned_ckey", &self.unbanned_ckey)?;
        state.end()
    }
}

pub async fn get_ban(
    ckey: Option<&str>,
    id: Option<&str>,
    pool: &MySqlPool,
) -> Result<Vec<Ban>, Error> {
    let mut connection = pool.acquire().await?;

    let query = {
        if let Some(ckey) = ckey {
            query("SELECT id, bantime, round_id, role, expiration_time, reason, ckey, a_ckey, edits, unbanned_datetime, unbanned_ckey FROM ban WHERE LOWER(ckey) = ?")
                .bind(ckey.to_lowercase())
        } else if let Some(id) = id {
            query("SELECT id, bantime, round_id, role, expiration_time, reason, ckey, a_ckey, edits, unbanned_datetime, unbanned_ckey FROM ban WHERE id = ?")
                .bind(id)
        } else {
            return Err(Error::NoCkeyOrId);
        }
    };

    let mut bans = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let ban = row?;

            let ban = Ban {
                id: ban.try_get("id")?,
                bantime: ban.try_get("bantime")?,
                round_id: ban.try_get("round_id")?,
                role: ban.try_get("role")?,
                expiration_time: ban.try_get("expiration_time")?,
                reason: ban.try_get("reason")?,
                ckey: ban.try_get("ckey")?,
                a_ckey: ban.try_get("a_ckey")?,
                edits: ban.try_get("edits")?,
                unbanned_datetime: ban.try_get("unbanned_datetime")?,
                unbanned_ckey: ban.try_get("unbanned_ckey")?,
            };

            bans.push(ban);
        }
    }

    connection.close().await?;

    Ok(bans)
}
