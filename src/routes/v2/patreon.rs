use rocket::{get, http::Status, State};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::{
    config::{self, Config},
    database::{error::Error, *},
    http::{
        self,
        discord::{get_guild_member, search_members},
    },
    Database,
};

use super::{common::ApiKey, Json};

#[get("/patreon?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    config: &State<Config>,
    _api_key: ApiKey,
) -> Result<Json<Value>, Status> {
    let Ok(patron) = is_patron(ckey, &database.pool, &config.discord, &config.proxy).await else {
        return Err(Status::InternalServerError);
    };

    Ok(Json::Ok(json!({ "patron": patron })))
}

async fn is_patron(
    ckey: &str,
    pool: &MySqlPool,
    discord: &config::Discord,
    proxy: &config::Proxy,
) -> Result<bool, Error> {
    let mut connection = pool.acquire().await?;

    let Ok(discord_id) = discord_id_by_ckey(ckey, &mut connection).await else {
        return Ok(false);
    };

    connection.close().await?;

    let member = match get_guild_member(discord.guild, discord_id, &discord.token, proxy).await {
        Ok(member) => member,
        Err(http::Error::Discord(10007 | 10013)) => {
            return Ok(false);
        }
        Err(e) => return Err(e)?,
    };

    Ok(member.roles.contains(&discord.patreon_role.to_string()))
}

#[get("/patreon/patrons")]
pub async fn patrons(
    database: &State<Database>,
    config: &State<Config>,
    _api_key: ApiKey,
) -> Result<Json<Value>, Status> {
    let Ok(patrons) = get_patrons(&database.pool, &config.discord, &config.proxy).await else {
        return Err(Status::InternalServerError);
    };

    Ok(Json::Ok(json!({ "patrons": patrons })))
}

async fn get_patrons(
    pool: &MySqlPool,
    discord: &config::Discord,
    proxy: &config::Proxy,
) -> Result<Vec<String>, Error> {
    let mut connection = pool.acquire().await?;

    let query = format!(
        "{{\"or_query\":{{}},\"and_query\":{{\"role_ids\":{{\"and_query\":[\"{}\"]}}}},\"limit\":1000}}",
        discord.patreon_role
    );

    let members = search_members(discord.guild, query, &discord.token, proxy).await?;

    let mut ckeys = Vec::new();

    for member in members {
        match ckey_by_discord_id(&member.user.id, &mut connection).await {
            Ok(ckey) => ckeys.push(ckey),
            Err(Error::NotLinked) => continue,
            Err(e) => return Err(e),
        }
    }

    connection.close().await?;

    Ok(ckeys)
}
