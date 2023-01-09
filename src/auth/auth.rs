use async_graphql::Error;
use chrono::{Duration, Utc};
use oauth2::AuthorizationCode;
use poem::web::cookie::Cookie;
use poem::web::{Data, Query, Redirect};
use poem::{handler, Body, IntoResponse, Request, Response, ResponseBuilder};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::system::core::Engine;
use crate::system::members::{MembersFilter, NewMemberPayload, NewMemberPayloadAuthKind};

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

const GITHUB_USER_API: &'static str = "https://api.github.com/user";

// pub async fn example_auth() {}

#[handler]
pub async fn github_sign_in(plexo_engine: Data<&Engine>) -> impl IntoResponse {
    let (url, _) = plexo_engine.0.auth.new_github_authorize_url();

    Redirect::temporary(url.to_string())
}

#[handler]
pub async fn github_callback(
    plexo_engine: Data<&Engine>,
    params: Query<GithubCallbackParams>,
) -> impl IntoResponse {
    let code = AuthorizationCode::new(params.code.clone());

    let gh_response = plexo_engine.auth.exchange_github_code(code).await;

    let Ok(access_token) = gh_response else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&gh_response).unwrap());
    };

    println!("token: {}", access_token);

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

    println!("github_user_data: {:#?}", github_user_data);

    let github_id = github_user_data
        .get("id")
        .unwrap()
        .as_i64()
        .unwrap()
        .to_string();

    let user_email = github_user_data
        .get("email")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let user_name = github_user_data
        .get("name")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let member = match plexo_engine
        .get_members(MembersFilter::new().set_github_id(github_id.to_string()))
        .await
        .first()
    {
        Some(member) => member.to_owned(),
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

    let Ok((access_token, refresh_token)) = plexo_engine
        .auth
        .jwt_engine
        .dispatch_jwt_access_refresh_pair(&member) else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Internal Server Error")).unwrap());
        };

    let mut cookie = Cookie::named("refresh-token");

    cookie.set_value_str(refresh_token);
    cookie.set_expires(Utc::now() + Duration::days(7));

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Set-Cookie", cookie.to_string())
        .body(
            Body::from_json(&AuthenticationResponse {
                access_token,
                token_type: None,
                scope: None,
            })
            .unwrap(),
        )
}

#[handler]
pub async fn refresh_token_handler(
    plexo_engine: Data<&Engine>,
    req: &Request,
) -> impl IntoResponse {
    let Some(token) = req.header("Authorization") else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Unauthorized")).unwrap());
    };

    let Some(access_token) = token.strip_prefix("Bearer ") else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Unauthorized")).unwrap());
    };

    let Ok(cookie) = Cookie::parse(req.header("Cookie").unwrap()) else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&Error::new("Unauthorized")).unwrap());
    };

    let refresh_token = cookie.value_str();

    let Ok(access_token) = plexo_engine
        .0
        .auth
        .refresh_token(access_token, refresh_token)
        .await
         else {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&Error::new("Unauthorized")).unwrap());
        };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            Body::from_json(&AuthenticationResponse {
                access_token,
                token_type: None,
                scope: None,
            })
            .unwrap(),
        )
}
