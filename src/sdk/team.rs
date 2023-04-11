use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use serde_json::de;
use uuid::Uuid;
use async_graphql::{
    dataloader::{DataLoader},
};
use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::{Member},
        project::Project,
    },
    system::core::Engine,
};

use super::loaders::{
    MemberLoader,
    ProjectLoader,
    LabelLoader,
    
};

#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct Team {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,

    pub owner_id: Uuid,

    pub visibility: TeamVisibility,

    pub prefix: Option<String>,
}

#[ComplexObject]
impl Team {
    pub async fn owner(&self, ctx: &Context<'_>) -> Option<Member> {
        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        //match to see is project_id is none
        let member = loader.load_one(self.owner_id).await.unwrap();
        match member {
            Some(member) => Some(member),
            None => None,
            
        }
                

    }

    pub async fn members(&self, ctx: &Context<'_>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        let ids : Vec<Uuid>= sqlx::query!(
            r#"
            SELECT member_id FROM members_by_teams
            WHERE team_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap().into_iter().map(|id| id.member_id).collect();

        
        let members_map = loader.load_many(ids.clone()).await.unwrap();

        let members: &Vec<Member> = &ids
            .into_iter()
            .map(|id|  {
                members_map.get(&id).unwrap().clone()
        } )
            .collect();

        members.clone()
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<ProjectLoader>>().unwrap();

        let ids : Vec<Uuid>= sqlx::query!(
            r#"
            SELECT project_id FROM teams_by_projects
            WHERE team_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap().into_iter().map(|id| id.project_id).collect();

        let projects_map = loader.load_many(ids.clone()).await.unwrap();

        let projects: &Vec<Project> = &ids
            .into_iter()
            .map(|id|  {
                projects_map.get(&id).unwrap().clone()
        } )
            .collect();

        projects.clone()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TeamVisibility {
    None,
    Public,
    Private,
    Internal,
}

impl TeamVisibility {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Public" => Self::Public,
            "Private" => Self::Private,
            "Internal" => Self::Internal,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Public => "Public",
            Self::Private => "Private",
            Self::Internal => "Internal",
        }
    }
}
