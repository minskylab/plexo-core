use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlexoAppError {
    #[error("Authorization token not provided")]
    MissingAuthorizationToken,
    #[error("Invalid authorization token")]
    InvalidAuthorizationToken,
    #[error("Email already in use")]
    EmailAlreadyInUse,
    #[error("Password isn't valid")]
    InvalidPassword,
    #[error("Email not found")]
    EmailNotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Poem error")]
    PoemError(#[from] poem::error::NotFoundError),
}
