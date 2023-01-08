use async_graphql::{Context, InputObject, Object};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::Member,
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::Team,
        utilities::DateTimeBridge,
    },
    system::core::Engine,
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
    async fn tasks(&self, ctx: &Context<'_>, filter: Option<TaskFilter>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let tasks = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, title, description, status, priority, due_date, project_id, assignee_id, labels, owner_id
            FROM tasks
            "#
        ).fetch_all(&plexo_engine.pool).await.unwrap();

        tasks
            .iter()
            .map(|r| Task {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                title: r.title.clone(),
                description: r.description.clone(),
                status: TaskStatus::from_optional_str(&r.status),
                priority: TaskPriority::from_optional_str(&r.priority),
                due_date: r.due_date.map(|d| DateTimeBridge::from_offset_date_time(d)),
                project_id: r.project_id,
                assignee_id: r.assignee_id,
                labels: r
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
                owner_id: r.owner_id.unwrap_or(Uuid::nil()),
            })
            .collect()
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

    async fn teams(&self) -> Vec<Team> {
        vec![]
    }

    async fn team_by_id(&self, id: Uuid) -> Team {
        todo!()
    }
}
