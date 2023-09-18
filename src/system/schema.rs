use async_graphql::{dataloader::DataLoader, Schema};

use crate::{
    graphql::{mutations::MutationRoot, queries::QueryRoot, subscription::SubscriptionRoot},
    sdk::loaders::{LabelLoader, MemberLoader, ProjectLoader, TaskLoader, TeamLoader},
    system::core::Engine,
};

pub trait GraphQLSchema {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
}

impl GraphQLSchema for Engine {
    fn graphql_api_schema(&self) -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            SubscriptionRoot,
        )
        .data(self.clone()) // TODO: Optimize this
        .data(DataLoader::new(TaskLoader::new(self.clone()), tokio::spawn))
        .data(DataLoader::new(
            ProjectLoader::new(self.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            LabelLoader::new(self.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            MemberLoader::new(self.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(TeamLoader::new(self.clone()), tokio::spawn))
        .finish()
    }
}
