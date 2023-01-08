use std::env;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, Client, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::Url;

use super::jwt::JWT;

#[derive(Clone)]
pub struct AuthEngine {
    pub jwt_engine: JWT,

    github_client: BasicClient,
    google_client: Option<BasicClient>,
}

impl AuthEngine {
    pub fn new() -> Self {
        let github_client_id = ClientId::new(
            env::var("GITHUB_CLIENT_ID")
                .expect("Missing the GITHUB_CLIENT_ID environment variable."),
        );
        let github_client_secret = ClientSecret::new(
            env::var("GITHUB_CLIENT_SECRET")
                .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Invalid token endpoint URL");

        let github_client = BasicClient::new(
            github_client_id,
            Some(github_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080/auth/github/callback".to_string())
                .expect("Invalid redirect URL"),
        );

        let jwt_access_token_secret = env::var("JWT_ACCESS_TOKEN_SECRET")
            .expect("Missing the JWT_ACCESS_TOKEN_SECRET environment variable.");

        let jwt_refresh_token_secret = env::var("JWT_REFRESH_TOKEN_SECRET")
            .expect("Missing the JWT_REFRESH_TOKEN_SECRET environment variable.");

        let jwt_engine = JWT::new(jwt_access_token_secret, jwt_refresh_token_secret);

        Self {
            jwt_engine,
            github_client,
            google_client: None,
        }
    }

    pub fn new_github_authorize_url(&self) -> (Url, CsrfToken) {
        self.github_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .url()
            .clone()

        // authorize_url.to_string()
    }

    pub async fn exchange_github_code(
        &self,
        code: AuthorizationCode,
        // state: CsrfToken,
    ) -> Result<String, String> {
        let token_result = self
            .github_client
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        match token_result {
            Ok(token) => Ok(token.access_token().secret().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn refresh_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        self.jwt_engine
            .refresh_access_token(access_token, refresh_token)
    }
}
