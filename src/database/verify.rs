use regex::Regex;
use sqlx::{pool::PoolConnection, query, Executor as _, MySql, MySqlPool, Row as _};

use super::error::Error;

pub async fn verify_discord(
    discord_id: &str,
    one_time_token: &str,
    pool: &MySqlPool,
) -> Result<String, Error> {
    let mut connection = pool.acquire().await?;

    if let Some(ckey) = get_ckey(discord_id, &mut connection).await? {
        return Err(Error::AlreadyLinked(ckey));
    }

    let regex = Regex::new(r"^([a-z']+-){5}[a-z']+$").unwrap();
    if !regex.is_match(one_time_token) {
        return Err(Error::TokenInvalid);
    }

    get_discord_id_by_token(one_time_token, false, &mut connection).await?;

    if let Ok(Some(discord_id)) =
        get_discord_id_by_token(one_time_token, true, &mut connection).await
    {
        return Err(Error::TokenInUse(discord_id));
    }

    let query =
        query("UPDATE discord_links SET discord_id = ?, valid = 1 WHERE one_time_token = ?");
    let query = query.bind(discord_id).bind(one_time_token);

    connection.execute(query).await?;

    let ckey = get_ckey(discord_id, &mut connection).await?;

    Ok(ckey.unwrap())
}

pub async fn unverify_discord(
    discord_id: Option<&str>,
    ckey: Option<&str>,
    pool: &MySqlPool,
) -> Result<String, Error> {
    let mut connection = pool.acquire().await?;

    if discord_id.is_some() {
        let discord_id = discord_id.unwrap();
        let ckey = get_ckey(discord_id, &mut connection).await?;

        if ckey.is_some() {
            let query =
                query("UPDATE discord_links SET valid = 0 WHERE discord_id = ? AND valid = 1");
            let query = query.bind(discord_id);

            connection.execute(query).await?;

            return Ok(ckey.unwrap());
        }

        return Err(Error::NotLinked);
    } else if ckey.is_some() {
        let ckey = ckey.unwrap();
        let discord_id = get_discord_id_by_ckey(ckey, &mut connection).await?;

        if discord_id.is_some() {
            let query =
                query("UPDATE discord_links SET valid = 0 WHERE LOWER(ckey) = ? AND valid = 1");
            let query = query.bind(ckey.to_lowercase());

            connection.execute(query).await?;

            return Ok(format!("@{}", discord_id.unwrap()));
        }

        return Err(Error::NotLinked);
    }

    Err(Error::InvalidArguments)
}

async fn get_ckey(
    discord_id: &str,
    connection: &mut PoolConnection<MySql>,
) -> Result<Option<String>, Error> {
    let query = query("SELECT ckey FROM discord_links WHERE discord_id = ? AND valid = 1");
    let bound = query.bind(discord_id);

    if let Ok(row) = connection.fetch_one(bound).await {
        return Ok(Some(row.try_get("ckey")?));
    }

    Ok(None)
}

async fn get_discord_id_by_ckey(
    ckey: &str,
    connection: &mut PoolConnection<MySql>,
) -> Result<Option<i64>, Error> {
    let query = query("SELECT discord_id FROM discord_links WHERE LOWER(ckey) = ? AND valid = 1");
    let bound = query.bind(ckey.to_lowercase());

    if let Ok(row) = connection.fetch_one(bound).await {
        return Ok(row.try_get("discord_id").ok());
    }

    Ok(None)
}

async fn get_discord_id_by_token(
    one_time_token: &str,
    only_valid: bool,
    connection: &mut PoolConnection<MySql>,
) -> Result<Option<i64>, Error> {
    let mut sql = "SELECT discord_id FROM discord_links WHERE one_time_token = ?".to_string();

    if only_valid {
        sql = format!("{sql} AND valid = 1");
    }

    let query = query(&sql).bind(one_time_token);

    if let Ok(row) = connection.fetch_one(query).await {
        return Ok(row.try_get("discord_id").ok());
    }

    Err(Error::TokenInvalid)
}
