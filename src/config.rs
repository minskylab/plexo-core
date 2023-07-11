use config::{Config, Environment};
use serde_derive::Deserialize;

use crate::errors::definitions::PlexoAppError;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub host: String,
    pub port: u32,
    pub url: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Github {
    // pub client_id: String,
    // pub client_secret: String,
    pub client: GithubClient,
    // pub redirect_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubClient {
    pub id: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct JWT {
    pub secret: String,
    // pub refresh_token_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct LLM {
    pub model_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PlexoAuthMethods {
    pub jwt: Option<JWT>,
    pub github: Option<Github>,
}

#[derive(Debug, Deserialize)]
pub struct PlexoConfig {
    pub service: Service,
    pub database: Database,
    pub auth: Option<PlexoAuthMethods>,
    // pub llm: LLM,
}

impl PlexoConfig {
    pub fn from_env() -> Result<Self, PlexoAppError> {
        let config = Config::builder()
            .set_default("service.host", "0.0.0.0")?
            .set_default("service.port", 8080)?
            .set_default("service.url", "x")?
            .set_default("service.domain", "x")?
            .set_default("auth.jwt.secret", "x")?
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .unwrap();

        let app: PlexoConfig = config.try_deserialize::<PlexoConfig>()?;

        Ok(app)
    }
}
