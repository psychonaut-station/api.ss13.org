use std::{collections::HashSet, sync::Arc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use super::{Error, REQWEST_CLIENT};

static DISCORD_API_LOCK: Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));

#[derive(Debug, Deserialize)]
struct ErrorMessage {
    code: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

pub async fn get_user(id: i64, token: &str) -> Result<User, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let url = format!("https://discord.com/api/v10/users/{id}");

    let response = REQWEST_CLIENT
        .get(url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?
        .text()
        .await?;

    let Ok(user) = serde_json::from_str(&response) else {
        let error: ErrorMessage = serde_json::from_str(&response)?;
        return Err(Error::Discord(error.code));
    };

    Ok(user)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMember {
    pub roles: HashSet<String>, // other fields are not required for now (https://discord.com/developers/docs/resources/guild#guild-member-object)
    pub member: Member,
}

pub async fn get_guild_member(
    guild_id: i64,
    user_id: i64,
    token: &str,
) -> Result<GuildMember, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let url = format!("https://discord.com/api/v10/guilds/{guild_id}/members/{user_id}");

    let response = REQWEST_CLIENT
        .get(url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?
        .text()
        .await?;

    let Ok(member) = serde_json::from_str(&response) else {
        let error: ErrorMessage = serde_json::from_str(&response)?;
        return Err(Error::Discord(error.code));
    };

    Ok(member)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMembersResponse {
    pub guild_id: String,
    pub members: Vec<GuildMember>,
}

pub async fn search_role_members(
    guild_id: i64,
    role_id: i64,
    token: &str,
) -> Result<GuildMembersResponse, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let url = format!("https://discord.com/api/v9/guilds/{guild_id}/members-search");

    let body = json!( {
        "or_query": {},
        "and_query": {
            "role_ids": {
                "and_query": [role_id.to_string()]
            }
        },
        "limit": 1000
    });

    let response = REQWEST_CLIENT
        .post(url)
        .header("Authorization", format!("Bot {token}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    let Ok(guild_members_response) = serde_json::from_str(&response) else {
        let error: ErrorMessage = serde_json::from_str(&response)?;
        return Err(Error::Discord(error.code));
    };

    Ok(guild_members_response)

}
