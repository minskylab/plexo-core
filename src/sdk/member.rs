use std::str::FromStr;

use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::Team,
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

use super::loaders::{ProjectLoader, TaskLoader, TeamLoader};

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
        println!("token: {}", auth_token);

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
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
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
        println!("token: {}", auth_token);

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
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
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
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<TaskLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT task_id FROM tasks_by_assignees
            WHERE assignee_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.task_id)
        .collect();

        let tasks_map = loader.load_many(ids.clone()).await.unwrap();

        let tasks: &Vec<Task> = &ids
            .into_iter()
            .map(|id| tasks_map.get(&id).unwrap().clone())
            .collect();

        tasks.clone()
    }

    pub async fn owned_projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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
                start_date: r.start_date.map(DateTimeBridge::from_offset_date_time),
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
            })
            .collect()
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<ProjectLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT project_id FROM members_by_projects
            WHERE member_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.project_id)
        .collect();

        let projects_map = loader.load_many(ids.clone()).await.unwrap();

        let projects: &Vec<Project> = &ids
            .into_iter()
            .map(|id| projects_map.get(&id).unwrap().clone())
            .collect();

        projects.clone()
    }

    pub async fn teams(&self, ctx: &Context<'_>) -> Option<Vec<Team>> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<TeamLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT team_id FROM members_by_teams
            WHERE member_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.team_id)
        .collect();

        let teams_map = loader.load_many(ids.clone()).await.unwrap();

        let teams: &Vec<Team> = &ids
            .into_iter()
            .map(|id| teams_map.get(&id).unwrap().clone())
            .collect();

        Some(teams.clone())
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
            Some(s) => Self::from_str(s.as_str()).unwrap_or(Self::ReadOnly),
            None => Self::ReadOnly,
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

impl FromStr for MemberRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Self::Admin),
            "Member" => Ok(Self::Member),
            "ReadOnly" => Ok(Self::ReadOnly),
            _ => Err(()),
        }
    }
}
