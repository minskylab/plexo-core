use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};


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
    pub async fn owned_tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let tasks = sqlx::query!(r#"SELECT * FROM tasks WHERE owner_id = $1"#, &self.id).fetch_all(&plexo_engine.pool).await.unwrap();
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

    pub async fn assigned_tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let tasks = sqlx::query!(r#"SELECT * FROM tasks WHERE assignee_id = $1"#, &self.id).fetch_all(&plexo_engine.pool).await.unwrap();
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

    pub async fn owned_projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let projects = sqlx::query!(r#"SELECT * FROM projects WHERE owner_id = $1"#, &self.id).fetch_all(&plexo_engine.pool).await.unwrap();
        projects
            .iter()
            .map(|r| Project {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                description: None,
                prefix: r.prefix.clone(),
                owner_id: r.owner_id.unwrap_or(Uuid::nil()),
            })
            .collect()
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
