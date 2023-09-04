use async_graphql::MergedObject;

use self::{ai_functions::AIFunctionsQuery, resources::ResourcesQuery};

pub mod ai_functions;
pub mod resources;

// use self::{auth::AuthMutation, resources::ResourcesMutation};

// use super::{auth_mutation:i:AuthMutation, resources_mutation::ResourcesMutation};

#[derive(MergedObject, Default)]
pub struct QueryRoot(ResourcesQuery, AIFunctionsQuery);
