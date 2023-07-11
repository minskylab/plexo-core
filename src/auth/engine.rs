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

// use crate::config::{JWT_ACCESS_TOKEN_SECRET, JWT_REFRESH_TOKEN_SECRET};

use super::{
    core::PlexoAuthToken,
    jwt::{JWTEngine, PlexoAuthTokenClaims},
};

#[derive(Clone)]
pub struct AuthEngine {
    pub jwt_engine: JWTEngine,

    github_client: Option<BasicClient>,
    password_hasher: Option<Argon2<'static>>,
    _google_client: Option<BasicClient>,
}

pub struct AuthEngineBuilder {
    github_client_id: Option<&'static str>,
    github_client_secret: Option<&'static str>,
    github_redirect_url: Option<&'static str>,

    password_hasher: Option<Argon2<'static>>,
}

impl AuthEngineBuilder {
    pub fn new() -> &'static Self {
        &Self {
            github_client_id: None,
            github_client_secret: None,
            github_redirect_url: None,
            password_hasher: None,
        }
    }

    pub fn with_github_method(
        &mut self,
        github_client_id: &'static str,
        github_client_secret: &'static str,
        github_redirect_url: &'static str,
    ) -> &Self {
        self.github_client_id = Some(github_client_id);
        self.github_client_secret = Some(github_client_secret);
        self.github_redirect_url = Some(github_redirect_url);

        self
    }

    pub fn with_email_password_method(&mut self) -> &Self {
        self.password_hasher = Some(Argon2::default());

        self
    }

    pub fn with_google_method(&mut self) -> &Self {
        unimplemented!();
    }

    pub fn build(&mut self) -> AuthEngine {
        // let jwt_access_token_secret = JWT_ACCESS_TOKEN_SECRET.clone();

        // let jwt_refresh_token_secret = JWT_REFRESH_TOKEN_SECRET.clone();
        todo!();

        // let jwt_engine = JWTEngine::new(jwt_access_token_secret, jwt_refresh_token_secret);

        // let mut auth_engine = AuthEngine {
        //     jwt_engine,

        //     github_client: None,
        //     password_hasher: None,
        //     _google_client: None,
        // };

        // if self.github_client_id.is_some()
        //     && self.github_client_secret.is_some()
        //     && self.github_redirect_url.is_some()
        // {
        //     let github_client_id = ClientId::new(self.github_client_id.unwrap().to_string());
        //     let github_client_secret =
        //         ClientSecret::new(self.github_client_secret.unwrap().to_string());

        //     let github_auth_url = "https://github.com/login/oauth/authorize".to_string();
        //     let github_token_url = "https://github.com/login/oauth/access_token".to_string();

        //     let auth_url =
        //         AuthUrl::new(github_auth_url).expect("Invalid authorization endpoint URL");
        //     let token_url = TokenUrl::new(github_token_url).expect("Invalid token endpoint URL");

        //     let github_client = BasicClient::new(
        //         github_client_id,
        //         Some(github_client_secret),
        //         auth_url,
        //         Some(token_url),
        //     )
        //     .set_redirect_uri(
        //         RedirectUrl::new(self.github_redirect_url.unwrap().to_string())
        //             .expect("Invalid redirect URL"),
        //     );

        //     auth_engine.github_client = Some(github_client);
        // }

        // if self.password_hasher.is_some() {
        //     auth_engine.password_hasher = self.password_hasher.clone();
        // }

        // auth_engine
    }
}

// impl Default for AuthEngineBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl AuthEngine {
    pub fn available_methods(&self) -> Vec<String> {
        let mut methods = vec![];

        if self.github_client.is_some() {
            methods.push("github".to_string());
        }

        if self.password_hasher.is_some() {
            methods.push("email".to_string());
        }

        if self._google_client.is_some() {
            methods.push("google".to_string());
        }

        methods
    }

    pub fn new_github_authorize_url(&self) -> Option<(Url, CsrfToken)> {
        match self.github_client.as_ref() {
            Some(client) => {
                let (authorize_url, csrf_token) = client
                    .authorize_url(CsrfToken::new_random)
                    .add_scope(Scope::new("user:email".to_string()))
                    .url();

                Some((authorize_url, csrf_token))
            }
            None => None,
        }
    }

    pub async fn exchange_github_code(
        &self,
        code: AuthorizationCode,
        _state: CsrfToken,
    ) -> Option<Result<String, String>> {
        let Some(client) = self.github_client.as_ref() else {
            return None;
        };

        let token_result = client
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        match token_result {
            Ok(token) => Some(Ok(token.access_token().secret().to_string())),
            Err(e) => Some(Err(e.to_string())),
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
}
