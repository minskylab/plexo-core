use std::process::id;

use async_graphql::{Context, InputType, Object, ComplexObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::{Pool, Postgres, query, types::time::OffsetDateTime};

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::Member,
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: Option<String>,
        owner_id: Uuid,
        status: String,
        priority: String,
        due_date: Option<DateTime<Utc>>,
        project_id: Uuid,
        assignee_id: Option<Uuid>,
        labels: Option<serde_json::Value>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        
        let task = sqlx::query!(
        r#"
        INSERT INTO tasks
        (title, description, owner_id, status, priority, due_date, project_id, assignee_id, labels)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, created_at, updated_at, title, description, owner_id, status, priority, due_date, project_id, assignee_id, labels;
        "#,
            title,
            description,
            owner_id,
            status,
            priority,
            due_date.map(|d| DateTimeBridge::from_date_time(d)), 
            project_id,
            assignee_id,
            labels,
        ).fetch_one(&plexo_engine.pool).await.unwrap();
        
        Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title.clone(),
            description: task.description.clone(),
            status: TaskStatus::from_optional_str(&task.status),
            priority: TaskPriority::from_optional_str(&task.priority),
            due_date: task
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task.project_id,
            assignee_id: task.assignee_id,
            labels: task
                .labels
                .as_ref()
                .map(|l| {
                    l.as_array()
                        .unwrap()
                        .iter()
                        .map(|s| s.as_str().unwrap().to_string())
                        .collect()
                })
                .unwrap_or(vec![]),
            owner_id: task.owner_id.unwrap_or(Uuid::nil()),
        }
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