use cookie::Cookie;
use poem::http::HeaderMap;

use crate::auth::core::{PlexoAuthToken, COOKIE_SESSION_TOKEN_NAME};

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| PlexoAuthToken(s.to_string())).ok())
}

pub fn get_token_from_cookie(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    let raw_cookie = headers.get("Cookie").and_then(|c| c.to_str().ok())?;

    get_token_from_raw_cookie(raw_cookie)
}

pub fn get_token_from_raw_cookie(raw_cookie: &str) -> Option<PlexoAuthToken> {
    for cookie in Cookie::split_parse(raw_cookie) {
        let Ok(cookie) = cookie else  {
            println!("Error parsing cookie");
            return None;
        };

        if cookie.name() == COOKIE_SESSION_TOKEN_NAME {
            return Some(PlexoAuthToken(cookie.value().to_string()));
        }
    }

    None
}
