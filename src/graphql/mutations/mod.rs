pub mod auth;
pub mod resources;

use async_graphql::MergedObject;

use self::{auth::AuthMutation, resources::ResourcesMutation};

// use super::{auth_mutation:i:AuthMutation, resources_mutation::ResourcesMutation};

#[derive(MergedObject, Default)]
pub struct MutationRoot(ResourcesMutation, AuthMutation);
