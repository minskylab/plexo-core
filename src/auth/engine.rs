use std::env;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};

use reqwest::Url;

use super::{
    core::PlexoAuthToken,
    jwt::{JWTEngine, PlexoAuthTokenClaims},
};

#[derive(Clone)]
pub struct AuthEngine {
    pub jwt_engine: JWTEngine,

    github_client: BasicClient,
    _google_client: Option<BasicClient>,
}

impl AuthEngine {
    pub fn new(
        github_client_id: &str,
        github_client_secret: &str,
        github_redirect_url: &str,
    ) -> Self {
        let github_client_id = ClientId::new(github_client_id.to_string());
        let github_client_secret = ClientSecret::new(github_client_secret.to_string());

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
            RedirectUrl::new(github_redirect_url.to_string()).expect("Invalid redirect URL"),
        );

        let jwt_access_token_secret = env::var("JWT_ACCESS_TOKEN_SECRET")
            .expect("Missing the JWT_ACCESS_TOKEN_SECRET environment variable.");

        let jwt_refresh_token_secret = env::var("JWT_REFRESH_TOKEN_SECRET")
            .expect("Missing the JWT_REFRESH_TOKEN_SECRET environment variable.");

        let jwt_engine = JWTEngine::new(jwt_access_token_secret, jwt_refresh_token_secret);

        Self {
            jwt_engine,
            github_client,
            _google_client: None,
        }
    }

    pub fn new_github_authorize_url(&self) -> (Url, CsrfToken) {
        self.github_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .url()
    }

    pub async fn exchange_github_code(
        &self,
        code: AuthorizationCode,
        _state: CsrfToken,
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

    pub async fn extract_claims_from_session_token(
        &self,
        session_token: &PlexoAuthToken,
    ) -> PlexoAuthTokenClaims {
        self.jwt_engine
            .decode_session_token(session_token.0.as_str())
            .unwrap()
    }

    // pub async fn extract_claims_from_access_token(
    //     &self,
    //     access_token: &PlexoAuthToken,
    // ) -> PlexoAuthTokenClaims {
    //     self.jwt_engine
    //         .decode_access_token(access_token.0.as_str())
    //         .unwrap()
    // }

    // pub async fn refresh_token(
    //     &self,
    //     access_token: &str,
    //     refresh_token: &str,
    // ) -> Result<String, jsonwebtoken::errors::Error> {
    //     self.jwt_engine
    //         .refresh_access_token(access_token, refresh_token)
    // }
}
