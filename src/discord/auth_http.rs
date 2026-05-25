use super::fingerprint::discord_web_user_agent;

pub(super) const DISCORD_ORIGIN: &str = "https://discord.com";
pub(super) const DISCORD_LOGIN_REFERER: &str = "https://discord.com/login";

pub(super) fn discord_web_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent(discord_web_user_agent())
        .build()
}

pub(super) fn discord_login_headers() -> reqwest::header::HeaderMap {
    use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, ORIGIN, REFERER};

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(ORIGIN, HeaderValue::from_static(DISCORD_ORIGIN));
    headers.insert(REFERER, HeaderValue::from_static(DISCORD_LOGIN_REFERER));
    headers.insert("X-Discord-Locale", HeaderValue::from_static("en-US"));
    headers
}
