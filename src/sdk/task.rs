use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use uuid::Uuid;

use super::{
    member::{Member, MemberRole},
    project::Project,
};
use crate::{auth::auth::PlexoAuthToken, sdk::utilities::DateTimeBridge, system::core::Engine};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,

    pub owner_id: Uuid,

    pub status: TaskStatus,
    pub priority: TaskPriority,

    pub due_date: Option<DateTime<Utc>>,

    pub project_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    
    pub labels: Vec<String>,
    pub count: i32,

    
}

#[ComplexObject]
impl Task {
    pub async fn owner(&self, ctx: &Context<'_>) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let member = sqlx::query!(r#"SELECT * FROM members WHERE id = $1"#, &self.owner_id)
            .fetch_one(&plexo_engine.pool)
            .await
            .unwrap();

        Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name.clone(),
            email: member.email.clone(),
            github_id: member.github_id.clone(),
            google_id: member.google_id.clone(),
            photo_url: member.photo_url.clone(),
            role: MemberRole::from_optional_str(&member.role),
        }
    }

    pub async fn leader(&self, ctx: &Context<'_>) -> Option<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        if self.lead_id.is_none() {
            return None;
        }

        let member = sqlx::query!(
            r#"SELECT * FROM members WHERE id = $1"#,
            &self.lead_id.unwrap()
        )
        .fetch_one(&plexo_engine.pool)
        .await;

        match member {
            Ok(member) => Some(Member {
                id: member.id,
                created_at: DateTimeBridge::from_offset_date_time(member.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
                name: member.name.clone(),
                email: member.email.clone(),
                github_id: member.github_id.clone(),
                google_id: member.google_id.clone(),
                photo_url: member.photo_url.clone(),
                role: MemberRole::from_optional_str(&member.role),
            }),
            Err(_) => None,
        }
    }

    pub async fn project(&self, ctx: &Context<'_>) -> Option<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        if self.project_id.is_none() {
            return None;
        }

        let project = sqlx::query!(
            r#"SELECT * FROM projects WHERE id = $1"#,
            &self.project_id.unwrap()
        )
        .fetch_one(&plexo_engine.pool)
        .await;

        match project {
            Ok(project) => Some(Project {
                id: project.id,
                created_at: DateTimeBridge::from_offset_date_time(project.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
                name: project.name.clone(),
                description: project.description.clone(),
                prefix: project.prefix.clone(),
                owner_id: project.owner_id.unwrap_or(Uuid::nil()),
                lead_id: project.lead_id,
                start_date: project
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
                due_date: project
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
            }),
            Err(_) => None,
        }
    }
    
    pub async fn assignees (&self, ctx: &Context<'_>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let members = sqlx::query!(r#"
        SELECT * FROM tasks_by_assignees JOIN members
        ON tasks_by_assignees.assignee_id = members.id WHERE task_id = $1"#,
         &self.id)
         .fetch_all(&plexo_engine.pool).await.unwrap();

        members
            .iter()
            .map(|r| Member {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                email: r.email.clone(),
                github_id: r.github_id.clone(),
                google_id: r.google_id.clone(),
                photo_url: r.photo_url.clone(),
                role: MemberRole::from_optional_str(&r.role),
            })
            .collect()
    } 


}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
    None,
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

impl TaskStatus {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Backlog" => Self::Backlog,
            "ToDo" => Self::ToDo,
            "InProgress" => Self::InProgress,
            "Done" => Self::Done,
            "Canceled" => Self::Canceled,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Backlog => "Backlog",
            Self::ToDo => "ToDo",
            Self::InProgress => "InProgress",
            Self::Done => "Done",
            Self::Canceled => "Canceled",
        }
    }
}

impl TaskPriority {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Low" => Self::Low,
            "Medium" => Self::Medium,
            "High" => Self::High,
            "Urgent" => Self::Urgent,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }
}
