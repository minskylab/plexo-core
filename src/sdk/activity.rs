use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Enum, Result, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use std::str::FromStr;

use super::loaders::MemberLoader;
use super::member::Member;
use crate::graphql::auth::extract_context;

#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct Activity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub member_id: Uuid,
    pub resource_id: Uuid,

    pub operation: ActivityOperationType,
    pub resource_type: ActivityResourceType,
}

#[ComplexObject]
impl Activity {
    pub async fn member(&self, ctx: &Context<'_>) -> Result<Member> {
        let (_plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<MemberLoader>>()?;

        Ok(loader.load_one(self.member_id).await?.unwrap())
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum ActivityOperationType {
    Create,
    Update,
    Delete,
}

impl ToString for ActivityOperationType {
    fn to_string(&self) -> String {
        match self {
            ActivityOperationType::Create => "Create".to_string(),
            ActivityOperationType::Update => "Update".to_string(),
            ActivityOperationType::Delete => "Delete".to_string(),
        }
    }
}

impl FromStr for ActivityOperationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Create" => Ok(ActivityOperationType::Create),
            "Update" => Ok(ActivityOperationType::Update),
            "Delete" => Ok(ActivityOperationType::Delete),
            _ => Err(()),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum ActivityResourceType {
    Task,
    Project,
    Team,
    Member,
    Label,
    Organization,
}

impl ToString for ActivityResourceType {
    fn to_string(&self) -> String {
        match self {
            ActivityResourceType::Task => "Task".to_string(),
            ActivityResourceType::Project => "Project".to_string(),
            ActivityResourceType::Team => "Team".to_string(),
            ActivityResourceType::Member => "Member".to_string(),
            ActivityResourceType::Label => "Label".to_string(),
            ActivityResourceType::Organization => "Organization".to_string(),
        }
    }
}

impl FromStr for ActivityResourceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Task" => Ok(ActivityResourceType::Task),
            "Project" => Ok(ActivityResourceType::Project),
            "Team" => Ok(ActivityResourceType::Team),
            "Member" => Ok(ActivityResourceType::Member),
            "Label" => Ok(ActivityResourceType::Label),
            "Organization" => Ok(ActivityResourceType::Organization),
            _ => Err(()),
        }
    }
}
