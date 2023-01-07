use oauth2::AuthorizationCode;
use poem::web::{Data, Query, Redirect};
use poem::{handler, Body, IntoResponse, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::system::core::Engine;

pub struct PlexoAuthToken(pub String);

const GITHUB_USER_API: &'static str = "https://api.github.com/user";

// pub async fn example_auth() {}

#[handler]
pub async fn github_sign_in(plexo_engine: Data<&Engine>) -> impl IntoResponse {
    let (url, _) = plexo_engine.0.auth.new_github_authorize_url();

    Redirect::temporary(url.to_string())
}

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

#[handler]
pub async fn github_callback(
    plexo_engine: Data<&Engine>,
    params: Query<GithubCallbackParams>,
) -> impl IntoResponse {
    let code = AuthorizationCode::new(params.code.clone());

    let token = plexo_engine.0.auth.exchange_github_code(code).await;

    match token {
        Ok(token) => {
            let access_token = token;

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

            // Redirect::temporary()
            // "success".to_string().with_status(StatusCode::OK)

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
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&e).unwrap()),
    }
}
