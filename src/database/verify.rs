use regex::Regex;
use sqlx::{pool::PoolConnection, query, Executor as _, MySql, MySqlPool, Row as _};

use crate::http::discord::{self, User};

use super::error::Error;

pub async fn verify_discord(
    discord_id: &str,
    one_time_token: &str,
    pool: &MySqlPool,
) -> Result<String, Error> {
    let mut connection = pool.acquire().await?;

    if let Ok(ckey) = get_ckey_by_discord_id(discord_id, &mut connection).await {
        return Err(Error::AlreadyLinked(ckey));
    }

    let regex = Regex::new(r"^([a-z']+-){5}[a-z']+$").unwrap();
    if !regex.is_match(one_time_token) {
        return Err(Error::TokenInvalid);
    }

    if let Err(Error::TokenInvalid) =
        get_discord_id_by_token(one_time_token, false, &mut connection).await
    {
        return Err(Error::TokenInvalid);
    }

    if let Ok(discord_id) = get_discord_id_by_token(one_time_token, true, &mut connection).await {
        return Err(Error::TokenInUse(discord_id));
    }

    let query =
        sqlx::query("UPDATE discord_links SET discord_id = ?, valid = 1 WHERE one_time_token = ?");
    let query = query.bind(discord_id).bind(one_time_token);

    connection.execute(query).await?;

    let ckey = get_ckey_by_discord_id(discord_id, &mut connection).await?;

    Ok(ckey)
}

pub async fn unverify_discord(
    discord_id: Option<&str>,
    ckey: Option<&str>,
    pool: &MySqlPool,
) -> Result<String, Error> {
    let mut connection = pool.acquire().await?;

    if let Some(discord_id) = discord_id {
        match get_ckey_by_discord_id(discord_id, &mut connection).await {
            Ok(ckey) => {
                let query = sqlx::query(
                    "UPDATE discord_links SET valid = 0 WHERE discord_id = ? AND valid = 1",
                );
                let query = query.bind(discord_id);

                connection.execute(query).await?;

                return Ok(ckey);
            }
            Err(e) => return Err(e),
        }
    } else if let Some(ckey) = ckey {
        match get_discord_id_by_ckey(ckey, &mut connection).await {
            Ok(discord_id) => {
                let query = sqlx::query(
                    "UPDATE discord_links SET valid = 0 WHERE LOWER(ckey) = ? AND valid = 1",
                );
                let query = query.bind(ckey.to_lowercase());

                connection.execute(query).await?;

                return Ok(format!("@{}", discord_id));
            }
            Err(e) => return Err(e),
        }
    }

    Err(Error::InvalidArguments)
}

async fn get_ckey_by_discord_id(
    discord_id: &str,
    connection: &mut PoolConnection<MySql>,
) -> Result<String, Error> {
    let query = sqlx::query("SELECT ckey FROM discord_links WHERE discord_id = ? AND valid = 1");
    let bound = query.bind(discord_id);

    if let Ok(row) = connection.fetch_one(bound).await {
        return Ok(row.try_get("ckey")?);
    }

    Err(Error::NotLinked)
}

pub async fn get_discord_id_by_ckey(
    ckey: &str,
    connection: &mut PoolConnection<MySql>,
) -> Result<i64, Error> {
    let query =
        sqlx::query("SELECT discord_id FROM discord_links WHERE LOWER(ckey) = ? AND valid = 1");
    let query = query.bind(ckey.to_lowercase());

    if let Ok(row) = connection.fetch_one(query).await {
        return Ok(row.try_get("discord_id")?);
    }

    let query = sqlx::query("SELECT 1 FROM player WHERE LOWER(ckey) = ?");
    let query = query.bind(ckey.to_lowercase());

    if connection.fetch_one(query).await.is_err() {
        return Err(Error::PlayerNotFound);
    }

    Err(Error::NotLinked)
}

async fn get_discord_id_by_token(
    one_time_token: &str,
    only_valid: bool,
    connection: &mut PoolConnection<MySql>,
) -> Result<i64, Error> {
    let mut sql = "SELECT discord_id FROM discord_links WHERE one_time_token = ?".to_string();

    if only_valid {
        sql = format!("{sql} AND valid = 1");
    }

    let query = query(&sql).bind(one_time_token);

    if let Ok(row) = connection.fetch_one(query).await {
        return row.try_get("discord_id").map_err(|_| Error::NotLinked);
    }

    Err(Error::TokenInvalid)
}

pub async fn fetch_discord_by_ckey(
    ckey: &str,
    discord_token: &str,
    pool: &MySqlPool,
) -> Result<User, Error> {
    let mut connection = pool.acquire().await?;

    match get_discord_id_by_ckey(ckey, &mut connection).await {
        Ok(discord_id) => Ok(discord::get_user(&discord_id.to_string(), discord_token).await?),
        Err(e) => Err(e),
    }
}
