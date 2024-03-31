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

pub async fn get_user(id: &str, token: &str) -> Result<User, Error> {
    let url = format!("https://discord.com/api/v10/users/{id}");

    let response = REQWEST_CLIENT
        .get(url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?;

    let user: User = response.json().await?;

    Ok(user)
}
