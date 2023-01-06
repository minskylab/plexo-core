use async_graphql::SimpleObject;
use sqlx::types::time::PrimitiveDateTime;

use super::{member::Member, project::Project};

pub enum TaskStatus {
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

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
    id: Uuid,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,

    title: String,
    description: Option<String>,

    status: TaskStatus,
    priority: TaskPriority,

    owner_id: Uuid,

    labels: Vec<String>,

    assignee_id: Option<Uuid>,
    project_id: Option<Uuid>,

    due_date: Option<PrimitiveDateTime>,
}

impl Task {
    pub fn owner() -> Member {
        todo!()
    }

    pub fn assignee() -> Option<Member> {
        todo!()
    }

    pub fn project() -> Option<Project> {
        todo!()
    }
}
