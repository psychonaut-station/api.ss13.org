use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{Error, REQWEST_CLIENT};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

pub async fn get_user(id: i64, token: &str) -> Result<User, Error> {
    let url = format!("https://discord.com/api/v10/users/{id}");

    let response = REQWEST_CLIENT
        .get(url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?;

    let user: User = response.json().await?;

    Ok(user)
}

#[derive(Debug, Deserialize)]
struct GuildMember {
    roles: HashSet<String>, // other fields are not required for now (https://discord.com/developers/docs/resources/guild#guild-member-object)
}

pub async fn is_patron(guild_id: i64, user_id: i64, role_id: i64, token: &str) -> Result<bool, Error> {
    let url = format!("https://discord.com/api/v10/guilds/{guild_id}/members/{user_id}");

    let response = REQWEST_CLIENT
        .get(url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(false);
    }

    let member: GuildMember = response.json().await?;

    if member.roles.contains(&role_id.to_string()) {
        return Ok(true);
    }

    Ok(false)
}
