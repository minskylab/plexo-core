use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenUrl,
};
use reqwest::Url;

pub struct AuthEngine {
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

        // Set up the config for the Github OAuth2 process.
        let github_client = BasicClient::new(
            github_client_id,
            Some(github_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        );

        Self {
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
}
