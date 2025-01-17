use rocket::{get, http::Status, State};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::{
    config::{self, Config},
    database::{error::Error, *},
    http::{self, discord::get_guild_member, discord::search_role_members},
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
    let Ok(patron) = is_patron(ckey, &database.pool, &config.discord).await else {
        return Err(Status::InternalServerError);
    };

    Ok(Json::Ok(json!({ "patron": patron })))
}

async fn is_patron(ckey: &str, pool: &MySqlPool, discord: &config::Discord) -> Result<bool, Error> {
    let mut connection = pool.acquire().await?;

    let Ok(discord_id) = discord_id_by_ckey(ckey, &mut connection).await else {
        return Ok(false);
    };

    connection.close().await?;

    let member = match get_guild_member(discord.guild, discord_id, &discord.token).await {
        Ok(member) => member,
        Err(http::Error::Discord(code)) => match code {
            10007 | 10013 => return Ok(false),
            _ => return Err(http::Error::Discord(code))?,
        },
        Err(e) => return Err(e)?,
    };

    Ok(member.roles.contains(&discord.patreon_role.to_string()))
}

#[get("/patrons")]
pub async fn patrons(
    database: &State<Database>,
    config: &State<Config>,
    _api_key: ApiKey,
) -> Result<Json<Value>, Status> {
    let discord = &config.discord;

    let user_ids: Vec<String> = match search_role_members(discord.guild, discord.patreon_role, &discord.token).await {
        Ok(response) => response.members.into_iter().map(|guild_member| guild_member.member.user.id).collect(),
        Err(_) => return Err(Status::InternalServerError)?,
    };

    let pool = &database.pool;

    let mut connection = pool.acquire().await.map_err(|_| Status::InternalServerError)?;

    let mut results = Vec::new();
    for user_id in user_ids {
        match ckey_by_discord_id(&user_id, &mut connection).await {
            Ok(result) => results.push(result),
            Err(Error::NotLinked) => continue,
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    connection.close().await.map_err(|_| Status::InternalServerError)?;


    Ok(Json::Ok(json!({ "patrons": results })))
}
