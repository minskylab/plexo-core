use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{project::Project, task::Task};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Member {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub email: String,

    pub github_id: Option<String>,
    pub google_id: Option<String>,

    pub photo_url: Option<String>,

    pub role: MemberRole,
}

#[ComplexObject]
impl Member {
    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }

    pub async fn assigned_tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        todo!()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum MemberRole {
    Admin,
    Member,
    ReadOnly,
}

impl MemberRole {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::ReadOnly,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Admin" => Self::Admin,
            "Member" => Self::Member,
            "ReadOnly" => Self::ReadOnly,
            _ => Self::ReadOnly,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Admin => "Admin",
            Self::Member => "Member",
            Self::ReadOnly => "ReadOnly",
        }
    }
}
