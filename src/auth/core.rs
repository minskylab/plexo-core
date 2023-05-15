use async_graphql::Error;
use chrono::{Duration, Utc};
use oauth2::{AuthorizationCode, CsrfToken};
use poem::web::cookie::{Cookie, SameSite};
use poem::web::{Data, Query, Redirect};
use poem::{handler, Body, IntoResponse, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::system::core::Engine;
use crate::system::members::{NewMemberPayload, NewMemberPayloadAuthKind};

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

#[handler]
pub async fn github_sign_in_handler(plexo_engine: Data<&Engine>) -> impl IntoResponse {
    let (url, _) = plexo_engine.0.auth.new_github_authorize_url();

    Redirect::temporary(url.to_string())
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

    // println!("github_user_data: {:#?}", github_user_data);

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
                .create_member(&NewMemberPayload::new(
                    NewMemberPayloadAuthKind::Github,
                    github_id,
                    user_email,
                    user_name,
                ))
                .await
        }
    };

    println!("member: {:?}", member);

    let Ok(session_token) = plexo_engine
        .auth
        .jwt_engine
        .create_session_token(&member) else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap());
        };

    let mut session_token_cookie = Cookie::named("__Host-plexo-session-token");

    session_token_cookie.set_value_str(session_token);
    session_token_cookie.set_http_only(true);
    session_token_cookie.set_secure(true);
    session_token_cookie.set_same_site(SameSite::Strict);
    session_token_cookie.set_expires(Utc::now() + Duration::days(7));
    session_token_cookie.set_path("/");

    Redirect::moved_permanent("/")
        .with_header("Set-Cookie", session_token_cookie.to_string())
        .into_response()
}

// #[handler]
// pub async fn refresh_token_handler(
//     plexo_engine: Data<&Engine>,
//     req: &Request,
// ) -> impl IntoResponse {
//     let unauthorized_response = Response::builder()
//         .status(StatusCode::UNAUTHORIZED)
//         .header("Content-Type", "application/json")
//         .body(Body::from_json(&Error::new("Unauthorized")).unwrap());

//     let Some(token) = req.header("Authorization") else {
//         return unauthorized_response;
//     };

//     let Some(access_token) = token.strip_prefix("Bearer ") else {
//         return unauthorized_response;
//     };

//     let Ok(cookie) = Cookie::parse(req.header("Cookie").unwrap()) else {
//         return unauthorized_response;
//     };

//     let refresh_token = cookie.value_str();

//     let Ok(access_token) = plexo_engine
//         .0
//         .auth
//         .refresh_token(access_token, refresh_token)
//         .await
//          else {
//             return Response::builder()
//                 .status(StatusCode::UNAUTHORIZED)
//                 .header("Content-Type", "application/json")
//                 .body(Body::from_json(&Error::new("Unauthorized")).unwrap());
//         };

//     Response::builder()
//         .status(StatusCode::OK)
//         .header("Content-Type", "application/json")
//         .body(
//             Body::from_json(AuthenticationResponse {
//                 access_token,
//                 token_type: None,
//                 scope: None,
//             })
//             .unwrap(),
//         )
// }

#[handler]
pub fn email_basic_login_handler() -> impl IntoResponse {
    "Hello World"
}
