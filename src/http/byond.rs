use super::{Error, REQWEST_CLIENT};

pub async fn is_member(ckey: &str) -> Result<bool, Error> {
    let response = REQWEST_CLIENT
        .get(format!(
            "https://secure.byond.com/members/{ckey}?format=text"
        ))
        .send()
        .await?;

    if let Some(content_type) = response.headers().get("content-type") {
        if let Ok(content_type) = content_type.to_str() {
            return Ok(content_type.split(';').next() == Some("text/plain"));
        }
    }

    Ok(false)
}
