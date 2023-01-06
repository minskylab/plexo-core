use std::env;

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use poem::web::{Data, Query, Redirect};
use poem::{handler, IntoResponse, Response, ResponseBuilder};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::system::core::Engine;

pub struct PlexoAuthToken(pub String);

pub async fn example_auth() {
    // A very naive implementation of the redirect server.
    // let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // loop {
    //     if let Ok((mut stream, _)) = listener.accept().await {
    //         let code;
    //         let state;
    //         {
    //             let mut reader = BufReader::new(&mut stream);

    //             let mut request_line = String::new();
    //             reader.read_line(&mut request_line).await.unwrap();

    //             let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    //             let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

    //             let code_pair = url
    //                 .query_pairs()
    //                 .find(|pair| {
    //                     let &(ref key, _) = pair;
    //                     key == "code"
    //                 })
    //                 .unwrap();

    //             let (_, value) = code_pair;
    //             code = AuthorizationCode::new(value.into_owned());

    //             let state_pair = url
    //                 .query_pairs()
    //                 .find(|pair| {
    //                     let &(ref key, _) = pair;
    //                     key == "state"
    //                 })
    //                 .unwrap();

    //             let (_, value) = state_pair;
    //             state = CsrfToken::new(value.into_owned());
    //         }

    //         let message = "Go back to your terminal :)";
    //         let response = format!(
    //             "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
    //             message.len(),
    //             message
    //         );
    //         stream.write_all(response.as_bytes()).await.unwrap();

    //         println!("Github returned the following code:\n{}\n", code.secret());
    //         println!(
    //             "Github returned the following state:\n{} (expected `{}`)\n",
    //             state.secret(),
    //             csrf_state.secret()
    //         );

    //         // Exchange the code with a token.
    //         let token_res = client
    //             .exchange_code(code)
    //             .request_async(async_http_client)
    //             .await;

    //         println!("Github returned the following token:\n{:?}\n", token_res);

    //         if let Ok(token) = token_res {
    //             let access_token = token.access_token().secret();

    //             println!("token: {}", access_token);
    //             println!("extra fields: {:#?}", token.extra_fields());

    //             // let introspection = client
    //             //     .introspect(token.access_token())
    //             //     .unwrap()
    //             //     .request_async(async_http_client)
    //             //     .await
    //             //     .unwrap();

    //             // println!("introspection: {:#?}", introspection);

    //             const USER_API: &'static str = "https://api.github.com/user";

    //             let client = reqwest::Client::new();

    //             let github_user_data = client
    //                 .get(USER_API)
    //                 .header("Authorization", format!("token {}", access_token))
    //                 .header("User-Agent", "plexo-agent")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .json::<Value>()
    //                 .await
    //                 .unwrap();

    //             println!("github_user_data: {:#?}", github_user_data);

    //             let scopes = if let Some(scopes_vec) = token.scopes() {
    //                 scopes_vec
    //                     .iter()
    //                     .map(|comma_separated| comma_separated.split(','))
    //                     .flatten()
    //                     .collect::<Vec<_>>()
    //             } else {
    //                 Vec::new()
    //             };
    //             println!("Github returned the following scopes:\n{:?}\n", scopes);
    //         }

    //         // The server will terminate itself after collecting the first code.
    //         break;
    //     }
    // }
}

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

#[handler]
pub async fn github_callback(
    plexo_engine: Data<&Engine>,
    params: Query<GithubCallbackParams>,
    // state: String,
) -> impl IntoResponse {
    let code = AuthorizationCode::new(params.code.clone());

    let token = plexo_engine.0.auth.exchange_github_code(code).await;

    match token {
        Ok(token) => {
            let access_token = token;

            println!("token: {}", access_token);
            // println!("extra fields: {:#?}", token.extra_fields());

            // let introspection = client
            //     .introspect(token.access_token())
            //     .unwrap()
            //     .request_async(async_http_client)
            //     .await
            //     .unwrap();

            // println!("introspection: {:#?}", introspection);

            const USER_API: &'static str = "https://api.github.com/user";

            let client = reqwest::Client::new();

            let github_user_data = client
                .get(USER_API)
                .header("Authorization", format!("token {}", access_token))
                .header("User-Agent", "plexo-agent")
                .send()
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            println!("github_user_data: {:#?}", github_user_data);

            // let scopes = if let Some(scopes_vec) = token.scopes() {
            //     scopes_vec
            //         .iter()
            //         .map(|comma_separated| comma_separated.split(','))
            //         .flatten()
            //         .collect::<Vec<_>>()
            // } else {
            //     Vec::new()
            // };
            // println!("Github returned the following scopes:\n{:?}\n", scopes);

            "success".to_string().with_status(StatusCode::OK)
        }
        Err(e) => format!("error: {}", e).with_status(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
