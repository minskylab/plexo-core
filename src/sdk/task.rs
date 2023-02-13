use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use uuid::Uuid;

use super::{member::{
    Member,
    MemberRole,
    },
     project::Project};
use crate::{
    system::core::Engine,
    auth::auth::PlexoAuthToken,
    sdk::{
        utilities::DateTimeBridge,
    
    },
};

#[derive(SimpleObject, Clone)]
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
    pub async fn owner(&self, ctx: &Context<'_>) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let member = sqlx::query!(
            r#"SELECT * FROM members WHERE id = $1"#,
            &self.owner_id
        )
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
            


    pub async fn assignee(&self, ctx: &Context<'_>) -> Option<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);
        
        if self.assignee_id.is_none() {
            return None
        }
        

        let member = sqlx::query!(
            r#"SELECT * FROM members WHERE id = $1"#,
            &self.assignee_id.unwrap()
        )
        .fetch_one(&plexo_engine.pool)
        .await;

        match member {
            Ok(member) => {
                Some(
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
                )
            }
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
            Ok(project) => {
                Some(
                    Project {
                        id: project.id,
                        created_at: DateTimeBridge::from_offset_date_time(project.created_at),
                        updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
                        name: project.name.clone(),
                        description: None,
                        prefix: project.prefix.clone(),
                        owner_id: project.owner_id.unwrap_or(Uuid::nil()),
                    }
                )
            }
            Err(_) => None,
        }
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
