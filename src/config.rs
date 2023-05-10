use std::env::var;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: String = var("HOST").unwrap_or("0.0.0.0".into());
    pub static ref PORT: String = var("PORT").unwrap_or("8080".into());
    pub static ref URL: String = var("URL").unwrap_or(format!("{}:{}", *HOST, *PORT));
    pub static ref DOMAIN: String = var("DOMAIN").unwrap_or(format!("http://{}", *URL));
    //
    pub static ref DATABASE_URL: String =
        var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    pub static ref GITHUB_CLIENT_ID: String =
        var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable.");
    pub static ref GITHUB_CLIENT_SECRET: String = var("GITHUB_CLIENT_SECRET")
        .expect("Missing the GITHUB_CLIENT_SECRET environment variable.");
    pub static ref GITHUB_REDIRECT_URL: String =
        var("GITHUB_REDIRECT_URL").unwrap_or(format!("{}/auth/github/callback", *DOMAIN));

    pub static ref LLM_MODEL_NAME: String = var("LLM_MODEL_NAME").unwrap_or("gpt-3.5-turbo".into());
}
