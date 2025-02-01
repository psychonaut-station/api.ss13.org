use std::{collections::HashSet, sync::Arc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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

pub async fn get_user(id: i64, token: &str, proxy: &crate::config::Proxy) -> Result<User, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let response = REQWEST_CLIENT
        .get(format!("https://{}/api/v10/users/{id}", proxy.discord))
        .header("X-PROXY-TOKEN", proxy.token.clone())
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
    // https://discord.com/developers/docs/resources/guild#guild-member-object
    pub roles: HashSet<String>,
    pub user: User,
}

pub async fn get_guild_member(
    guild_id: i64,
    user_id: i64,
    token: &str,
    proxy: &crate::config::Proxy,
) -> Result<GuildMember, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let response = REQWEST_CLIENT
        .get(format!(
            "https://{}/api/v10/guilds/{guild_id}/members/{user_id}",
            proxy.discord
        ))
        .header("X-PROXY-TOKEN", proxy.token.clone())
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

pub async fn search_members(
    guild_id: i64,
    query: String,
    token: &str,
    proxy: &crate::config::Proxy,
) -> Result<Vec<GuildMember>, Error> {
    let _lock = DISCORD_API_LOCK.lock().await;

    let response = REQWEST_CLIENT
        .post(format!(
            "https://{}/api/v10/guilds/{guild_id}/members-search",
            proxy.discord
        ))
        .header("X-PROXY-TOKEN", proxy.token.clone())
        .header("Authorization", format!("Bot {token}"))
        .header("Content-Type", "application/json")
        .body(query)
        .send()
        .await?
        .text()
        .await?;

    #[derive(Deserialize)]
    struct Response {
        pub members: Vec<ResponseMember>,
    }

    #[derive(Deserialize)]
    struct ResponseMember {
        pub member: GuildMember,
    }

    let Ok(response) = serde_json::from_str::<Response>(&response) else {
        let error: ErrorMessage = serde_json::from_str(&response)?;
        return Err(Error::Discord(error.code));
    };

    let members = response.members.into_iter().map(|m| m.member).collect();

    Ok(members)
}
