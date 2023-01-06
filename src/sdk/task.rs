use async_graphql::{scalar, ComplexObject, Enum, OutputType, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::types::time::PrimitiveDateTime;

use uuid::Uuid;

use super::{member::Member, project::Project};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskPriority {
    None,
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,

    pub status: TaskStatus,
    pub priority: TaskPriority,

    pub owner_id: Uuid,

    pub labels: Vec<String>,

    pub assignee_id: Option<Uuid>,
    pub project_id: Option<Uuid>,

    pub due_date: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Task {
    pub async fn owner(&self) -> Member {
        todo!()
    }

    pub async fn assignee(&self) -> Option<Member> {
        todo!()
    }

    pub async fn project(&self) -> Option<Project> {
        todo!()
    }
}
