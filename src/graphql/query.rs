use async_graphql::{InputObject, Object};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::sdk::{
    member::Member,
    project::Project,
    task::{Task, TaskPriority, TaskStatus},
};

pub struct QueryRoot;

#[derive(InputObject)]
pub struct TaskFilter {
    pub project_id: Option<Uuid>,
    pub assignee_id: Option<Uuid>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date_from: Option<DateTime<Utc>>,
    pub due_date_to: Option<DateTime<Utc>>,
}

#[Object]
impl QueryRoot {
    async fn tasks(&self, filter: Option<TaskFilter>) -> Vec<Task> {
        vec![]
    }

    async fn task_by_id(&self, id: Uuid) -> Task {
        todo!()
    }

    async fn members(&self) -> Vec<Member> {
        vec![]
    }

    async fn member_by_id(&self, id: Uuid) -> Member {
        todo!()
    }

    async fn member_by_email(&self, email: String) -> Member {
        todo!()
    }

    async fn projects(&self) -> Vec<Project> {
        vec![]
    }

    async fn project_by_id(&self, id: Uuid) -> Project {
        todo!()
    }
}
