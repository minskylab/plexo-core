use async_graphql::Error;
use chrono::{Duration, Utc};
use oauth2::{AuthorizationCode, CsrfToken};
use poem::http::header::SET_COOKIE;
use poem::http::HeaderMap;
use poem::web::cookie::{Cookie, SameSite};
use poem::web::{Data, Json, Query, Redirect};
use poem::{
    handler,
    http::{
        header::{CACHE_CONTROL, EXPIRES, LOCATION, PRAGMA},
        StatusCode,
    },
    Body, IntoResponse, Response, Result,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::commons::authorization::{get_token_from_cookie, get_token_from_headers};
use crate::errors::definitions::PlexoAppError;
use crate::system::core::Engine;

#[derive(Debug, Deserialize)]
pub struct GithubCallbackParams {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
    access_token: String,
    token_type: Option<String>,
    scope: Option<String>,
}

pub struct PlexoAuthToken(pub String);

const GITHUB_USER_API: &str = "https://api.github.com/user";
pub const COOKIE_SESSION_TOKEN_NAME: &str = "plexo-session-token";

#[handler]
pub async fn github_sign_in_handler(plexo_engine: Data<&Engine>) -> impl IntoResponse {
    let Some((url, _)) = plexo_engine.0.auth.new_github_authorize_url() else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error (github)")).unwrap());
    };

    Redirect::temporary(url.to_string())
        // .with_header("Set-Cookie", session_token_cookie.to_string())
        // .with_header(CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        // .with_header(PRAGMA, "no-cache")
        // .with_header(EXPIRES, "0")
        .into_response()
}

#[handler]
pub async fn github_callback_handler(
    plexo_engine: Data<&Engine>,
    params: Query<GithubCallbackParams>,
) -> impl IntoResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());

    let gh_response = plexo_engine.auth.exchange_github_code(code, state).await;

    let Ok(access_token) = gh_response else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&gh_response).unwrap());
    };

    let client = reqwest::Client::new();

    let github_user_data = client
        .get(GITHUB_USER_API)
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "plexo-agent")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let github_id: String = github_user_data
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap()
        .to_string();

    let user_email = github_user_data
        .get("email")
        .map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .unwrap_or(format!("{}@no-email.github.com", github_id.clone()))
        })
        .unwrap();

    let user_name = github_user_data
        .get("name")
        .map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .unwrap_or(github_id.clone())
        })
        .unwrap();

    let member: crate::sdk::member::Member = match plexo_engine
        .get_member_by_github_id(github_id.clone())
        .await
    {
        Some(member) => member,
        None => {
            plexo_engine
                .create_member_from_github(user_email, user_name, github_id)
                .await
        }
    };

    let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap());
    };

    let mut session_token_cookie = Cookie::named(COOKIE_SESSION_TOKEN_NAME);

    session_token_cookie.set_value_str(session_token);
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Lax);
    session_token_cookie.set_expires(Utc::now() + Duration::days(7));
    session_token_cookie.set_path("/");

    Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/")
        .header(CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .header(PRAGMA, "no-cache")
        .header(EXPIRES, "0")
        .header(SET_COOKIE, session_token_cookie.to_string())
        .body(Body::empty())
}

#[handler]
pub fn logout() -> impl IntoResponse {
    let mut session_token_cookie = Cookie::named(COOKIE_SESSION_TOKEN_NAME);

    session_token_cookie.set_value_str("");
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Strict);
    session_token_cookie.set_expires(Utc::now() - Duration::days(1));
    session_token_cookie.set_path("/");

    Redirect::moved_permanent("/")
        .with_header("Set-Cookie", session_token_cookie.to_string())
        .with_header(CACHE_CONTROL, "no-cache, no-store, must-revalidate")
        .with_header(PRAGMA, "no-cache")
        .with_header(EXPIRES, "0")
        .into_response()
}

#[derive(Debug, Deserialize)]
pub struct EmailLoginParams {
    pub email: String,
    pub password: String,
}

#[handler]
pub async fn email_basic_login_handler(
    plexo_engine: Data<&Engine>,
    params: Json<EmailLoginParams>,
) -> impl IntoResponse {
    let Some(member) = plexo_engine.get_member_by_email(params.email.clone()).await else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(
                Body::from_json(json!({
                    "error": "Member not found"
                }))
                .unwrap(),
            );
    };

    let Some(password_hash) = member.password_hash.clone() else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(
                Body::from_json(json!({
                    "error": "Invalid password"
                }))
                .unwrap(),
            );
    };

    if !plexo_engine
        .auth
        .validate_password(params.password.as_str(), password_hash.as_str())
    {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(
                Body::from_json(json!({
                    "error": "Invalid password"
                }))
                .unwrap(),
            );
    };

    let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap());
    };

    let mut session_token_cookie = Cookie::named(COOKIE_SESSION_TOKEN_NAME);

    session_token_cookie.set_value_str(session_token.clone());
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Lax);
    session_token_cookie.set_expires(Utc::now() + Duration::days(7));
    session_token_cookie.set_path("/");

    Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, session_token_cookie.to_string())
        .header("Content-Type", "application/json")
        .body(Body::from_json(json!({ "access_token": session_token })).unwrap())
}

#[derive(Debug, Deserialize)]
pub struct EmailRegisterParams {
    pub email: String,
    pub name: String,
    pub password: String,
}

fn _get_token(headers: &HeaderMap) -> Result<PlexoAuthToken, PlexoAppError> {
    if let Some(token) = get_token_from_headers(headers) {
        return Ok(token);
    }

    if let Some(token) = get_token_from_cookie(headers) {
        return Ok(token);
    }

    Err(PlexoAppError::PoemError(poem::error::NotFoundError))
}

#[handler]
pub async fn email_basic_register_handler(
    // headers: &HeaderMap,
    plexo_engine: Data<&Engine>,
    params: Json<EmailRegisterParams>,
) -> Result<Response> {
    // let token = get_token(headers)?;

    // let (plexo_engine, member_id) = extract_context(ctx)?;

    if (plexo_engine.get_member_by_email(params.email.clone()).await).is_some() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(
                Body::from_json(json!({
                    "error": "Member already exists"
                }))
                .unwrap(),
            ));
    };

    let password_hash = plexo_engine.auth.hash_password(params.password.as_str());

    let Some(member) = plexo_engine
        .create_member_from_email(params.email.clone(), params.name.clone(), password_hash)
        .await
    else {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap()));
    };

    let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap()));
    };

    let mut session_token_cookie = Cookie::named(COOKIE_SESSION_TOKEN_NAME);

    session_token_cookie.set_value_str(session_token.clone());
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Lax);
    session_token_cookie.set_expires(Utc::now() + Duration::days(7));
    session_token_cookie.set_path("/");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, session_token_cookie.to_string())
        .header("Content-Type", "application/json")
        .body(Body::from_json(json!({ "access_token": session_token })).unwrap()))
}

#[handler]
pub async fn logout_handler() -> Result<Response> {
    // plexo_engine: Data<&Engine>
    let mut session_token_cookie = Cookie::named(COOKIE_SESSION_TOKEN_NAME);

    session_token_cookie.set_value_str("");
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Strict);
    session_token_cookie.set_expires(Utc::now() - Duration::days(1));
    session_token_cookie.set_path("/");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, session_token_cookie.to_string())
        .header("Content-Type", "application/json")
        .body(Body::from_json(json!({ "access_token": "" })).unwrap()))
}
