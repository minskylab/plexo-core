use std::env::var;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: String = var("HOST").unwrap_or("0.0.0.0".into());
    pub static ref PORT: String = var("PORT").unwrap_or("8080".into());
    pub static ref URL: String = var("URL").unwrap_or(format!("{}:{}", *HOST, *PORT));
    pub static ref SCHEMA: String = var("SCHEMA").unwrap_or("http".into());
    pub static ref DOMAIN: String = var("DOMAIN").unwrap_or(format!("{}://{}", *SCHEMA, *URL));
    //
    pub static ref DATABASE_URL: String = var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    pub static ref GITHUB_CLIENT_ID: Option<String> = var("GITHUB_CLIENT_ID").ok();
    pub static ref GITHUB_CLIENT_SECRET: Option<String> = var("GITHUB_CLIENT_SECRET").ok();
    pub static ref GITHUB_REDIRECT_URL: String = var("GITHUB_REDIRECT_URL").unwrap_or(format!("{}/auth/github/callback", *DOMAIN));

    pub static ref LLM_MODEL_NAME: String = var("LLM_MODEL_NAME").unwrap_or("gpt-3.5-turbo".into());

    pub static ref ADMIN_EMAIL: String = var("ADMIN_EMAIL").unwrap_or("admin@plexo.app".into());
    pub static ref ADMIN_PASSWORD: String = var("ADMIN_PASSWORD").unwrap_or("admin".into());
    pub static ref ADMIN_NAME: String = var("ADMIN_NAME").unwrap_or("Admin".into());

    pub static ref ORGANIZATION_NAME: String = var("ORGANIZATION_NAME").unwrap_or("Plexo".into());

    pub static ref JWT_ACCESS_TOKEN_SECRET: String = var("JWT_ACCESS_TOKEN_SECRET").unwrap_or("secret".into());
    pub static ref JWT_REFRESH_TOKEN_SECRET: String = var("JWT_REFRESH_TOKEN_SECRET").unwrap_or("secret".into());

    pub static ref STATIC_PAGE_ENABLED: bool = var("STATIC_PAGE_ENABLED").unwrap_or("false".into()).to_lowercase() == "true";
}
