use async_graphql::{Context, Result};
use uuid::Uuid;

use crate::{auth::core::PlexoAuthToken, errors::definitions::PlexoAppError, system::core::Engine};

pub fn extract_context(ctx: &Context<'_>) -> Result<(Engine, Uuid)> {
    let Ok(auth_token) = &ctx.data::<PlexoAuthToken>() else {
        return Err(PlexoAppError:: MissingAuthorizationToken.into());
    };

    let plexo_engine = ctx.data::<Engine>()?.to_owned();

    let Ok(claims) = plexo_engine.auth.extract_claims(auth_token) else {
        return Err(PlexoAppError:: InvalidAuthorizationToken.into());
    };

    let member_id = claims.member_id();

    Ok((plexo_engine, member_id))
}
