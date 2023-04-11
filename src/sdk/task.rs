use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use uuid::Uuid;
use async_graphql::{
    dataloader::{DataLoader},
};

use super::{
    labels::Label,
    member::{Member},
    project::Project,
};

use crate::{auth::auth::PlexoAuthToken, system::core::Engine};
use super::loaders::{
    MemberLoader,
    ProjectLoader,
    LabelLoader,
    
};
#[derive(SimpleObject, Clone, Debug)]
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

    pub count: i32,
}

#[ComplexObject]
impl Task {
    pub async fn owner(&self, ctx: &Context<'_>) -> Option<Member> {
        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        //match to see is project_id is none
        let member = loader.load_one(self.owner_id).await.unwrap();
        match member {
            Some(member) => Some(member),
            None => None,
            
        }
                

    }

    pub async fn leader(&self, ctx: &Context<'_>) -> Option<Member> {
        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();
        
        //match to see is project_id is none
        match self.lead_id {
            Some(lead_id) => {
                let member = loader.load_one(lead_id).await.unwrap();
                match member {
                    Some(member) => Some(member),
                    None => None,
                    
                }
                
            },
            None => None,
        }
    }

    pub async fn project(&self, ctx: &Context<'_>) -> Option<Project> {
        let loader = ctx.data::<DataLoader<ProjectLoader>>().unwrap();
        
        //match to see is project_id is none
        match self.project_id {
            Some(project_id) => {
                let project = loader.load_one(project_id).await.unwrap();
                match project {
                    Some(project) => Some(project),
                    None => None,
                    
                }
                
            },
            None => None,
        }

    }

    pub async fn labels(&self, ctx: &Context<'_>) -> Vec<Label> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<LabelLoader>>().unwrap();

        let ids : Vec<Uuid>= sqlx::query!(
            r#"
            SELECT label_id FROM labels_by_tasks
            WHERE task_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap().into_iter().map(|id| id.label_id).collect();

        let labels_map = loader.load_many(ids.clone()).await.unwrap();

        let labels: &Vec<Label> = &ids
            .into_iter()
            .map(|id|  {
                labels_map.get(&id).unwrap().clone()
        } )
            .collect();

        labels.clone()       
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskStatus {
    None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
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
