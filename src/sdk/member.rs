use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
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
        let tasks = sqlx::query!(r#"SELECT * FROM tasks WHERE owner_id = $1"#, &self.id)
            .fetch_all(&plexo_engine.pool)
            .await
            .unwrap();
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
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
            })
            .collect()
    }

    pub async fn leading_tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let tasks = sqlx::query!(r#"SELECT * FROM tasks WHERE lead_id = $1"#, &self.id)
            .fetch_all(&plexo_engine.pool)
            .await
            .unwrap();
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
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
            })
            .collect()
    }

    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let tasks = sqlx::query!(
            r#"
        SELECT * FROM tasks_by_assignees JOIN tasks
        ON tasks_by_assignees.task_id = tasks.id WHERE assignee_id = $1"#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap();

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
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
            })
            .collect()
    }

    pub async fn owned_projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let projects = sqlx::query!(r#"SELECT * FROM projects WHERE owner_id = $1"#, &self.id)
            .fetch_all(&plexo_engine.pool)
            .await
            .unwrap();
        projects
            .iter()
            .map(|r| Project {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                description: r.description.clone(),
                prefix: r.prefix.clone(),
                owner_id: r.owner_id,
                lead_id: r.lead_id,
                start_date: r
                    .start_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
                due_date: r
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
            })
            .collect()
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let projects = sqlx::query!(
            r#"
        SELECT * FROM members_by_projects JOIN projects
        ON members_by_projects.project_id = projects.id WHERE member_id = $1"#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap();

        projects
            .iter()
            .map(|r| Project {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                description: r.description.clone(),
                prefix: r.prefix.clone(),
                owner_id: r.owner_id,
                lead_id: r.lead_id,
                start_date: r
                    .start_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
                due_date: r
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
            })
            .collect()
    }

    pub async fn teams(&self, ctx: &Context<'_>) -> Vec<Team> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let teams = sqlx::query!(
            r#"
        SELECT * FROM members_by_teams JOIN teams
        ON members_by_teams.team_id = teams.id WHERE member_id = $1"#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap();

        teams
            .iter()
            .map(|r| Team {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                owner_id: r.owner_id,
                visibility: TeamVisibility::from_optional_str(&r.visibility),
                prefix: r.prefix.clone(),
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
