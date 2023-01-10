use async_graphql::Object;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::sdk::{
    member::Member,
    project::Project,
    task::{Task, TaskPriority, TaskStatus},
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_task(
        &self,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        status: TaskStatus,
        project_id: Uuid,
        assignee_id: Option<Uuid>,
        due_date: Option<DateTime<Utc>>,
    ) -> Task {
        todo!()
    }

    async fn update_task(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
        priority: Option<TaskPriority>,
        status: Option<TaskStatus>,
        project_id: Option<Uuid>,
        assignee_id: Option<Uuid>,
        due_date: Option<DateTime<Utc>>,
    ) -> Task {
        todo!()
    }

    async fn delete_task(&self, id: Uuid) -> Task {
        todo!()
    }

    // async fn create_member(
    //     &self,
    //     email: String,
    //     password: String,
    //     first_name: String,
    //     last_name: String,
    // ) -> Member {
    //     todo!()
    // }

    async fn update_member(
        &self,
        id: Uuid,
        email: Option<String>,
        password: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Member {
        todo!()
    }

    // async fn delete_member(&self, id: Uuid) -> Member {
    //     todo!()
    // }

    async fn create_project(
        &self,
        title: String,
        description: Option<String>,
        owner_id: Uuid,
        labels: Vec<String>,
    ) -> Project {
        todo!()
    }

    async fn update_project(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
        owner_id: Option<Uuid>,
        labels: Option<Vec<String>>,
    ) -> Project {
        todo!()
    }

    async fn delete_project(&self, id: Uuid) -> Project {
        todo!()
    }
}
