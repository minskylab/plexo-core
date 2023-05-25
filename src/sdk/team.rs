use std::str::FromStr;

use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject};
use chrono::{DateTime, Utc};

use crate::{
    graphql::auth::extract_context,
    sdk::{member::Member, project::Project},
};
use async_graphql::dataloader::DataLoader;
use uuid::Uuid;

use super::loaders::{MemberLoader, ProjectLoader};

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
    pub async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Member>> {
        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        //match to see is project_id is none
        Ok(loader.load_one(self.owner_id).await.unwrap())
    }

    pub async fn members(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT member_id FROM members_by_teams
            WHERE team_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap()
        .into_iter()
        .map(|id| id.member_id)
        .collect();

        let members_map = loader.load_many(ids.clone()).await.unwrap();

        let members: &Vec<Member> = &ids
            .into_iter()
            .map(|id| members_map.get(&id).unwrap().clone())
            .collect();

        Ok(members.clone())
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<ProjectLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT project_id FROM teams_by_projects
            WHERE team_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*plexo_engine.pool)
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

        Ok(projects.clone())
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
            Some(s) => Self::from_str(s.as_str()).unwrap_or(Self::None),
            None => Self::None,
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

impl FromStr for TeamVisibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(Self::None),
            "Public" => Ok(Self::Public),
            "Private" => Ok(Self::Private),
            "Internal" => Ok(Self::Internal),
            _ => Err(()),
        }
    }
}
