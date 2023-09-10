use std::{env, error::Error};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
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

    github_client: Option<BasicClient>,
    _google_client: Option<BasicClient>,
}

impl AuthEngine {
    pub fn new(
        jwt_access_token_secret: String,
        jwt_refresh_token_secret: String,
        //
        github_client_id: Option<String>,
        github_client_secret: Option<String>,
        github_redirect_url: Option<String>,
    ) -> Self {
        let mut github_client: Option<BasicClient> = None;

        if let (Some(github_client_id), Some(github_client_secret), Some(github_redirect_url)) =
            (github_client_id, github_client_secret, github_redirect_url)
        {
            let github_client_id = ClientId::new(github_client_id.to_string());
            let github_client_secret = ClientSecret::new(github_client_secret.to_string());

            let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid authorization endpoint URL");
            let token_url =
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .expect("Invalid token endpoint URL");

            github_client = Some(
                BasicClient::new(
                    github_client_id,
                    Some(github_client_secret),
                    auth_url,
                    Some(token_url),
                )
                .set_redirect_uri(
                    RedirectUrl::new(github_redirect_url.to_string())
                        .expect("Invalid redirect URL"),
                ),
            );
        }

        // match (github_client_id, github_client_secret, github_redirect_url) {
        //     (Some(github_client_id), Some(github_client_secret), Some(github_redirect_url)) => {
        //         let github_client_id = ClientId::new(github_client_id.to_string());
        //         let github_client_secret = ClientSecret::new(github_client_secret.to_string());

        //         let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        //             .expect("Invalid authorization endpoint URL");
        //         let token_url =
        //             TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        //                 .expect("Invalid token endpoint URL");

        //         github_client = Some(
        //             BasicClient::new(
        //                 github_client_id,
        //                 Some(github_client_secret),
        //                 auth_url,
        //                 Some(token_url),
        //             )
        //             .set_redirect_uri(
        //                 RedirectUrl::new(github_redirect_url.to_string())
        //                     .expect("Invalid redirect URL"),
        //             ),
        //         );
        //     }
        //     _ => {}
        // }

        let jwt_engine = JWTEngine::new(
            jwt_access_token_secret.to_string(),
            jwt_refresh_token_secret.to_string(),
        );

        Self {
            jwt_engine,
            github_client,
            _google_client: None,
        }
    }

    pub fn new_github_authorize_url(&self) -> Option<(Url, CsrfToken)> {
        self.github_client.as_ref().map(|client| {
            client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("user:email".to_string()))
                .url()
        })
    }

    pub async fn exchange_github_code(
        &self,
        code: AuthorizationCode,
        _state: CsrfToken,
    ) -> Result<String, String> {
        let token_result = self
            .github_client
            .as_ref()
            .unwrap()
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        match token_result {
            Ok(token) => Ok(token.access_token().secret().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn extract_claims(
        &self,
        plexo_auth_token: &PlexoAuthToken,
    ) -> Result<PlexoAuthTokenClaims, Box<dyn Error>> {
        Ok(self
            .jwt_engine
            .decode_session_token(plexo_auth_token.0.as_str())?)
    }

    pub fn validate_password(&self, password: &str, password_hash: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn hash_password(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn has_github_client(&self) -> bool {
        self.github_client.is_some()
    }

    pub fn has_google_client(&self) -> bool {
        self._google_client.is_some()
    }
}
