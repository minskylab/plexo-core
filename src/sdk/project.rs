use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use async_graphql::dataloader::DataLoader;

use super::loaders::{MemberLoader, TeamLoader};
use crate::{
    graphql::auth::extract_context,
    sdk::{
        member::Member,
        task::{Task, TaskPriority, TaskStatus},
        team::Team,
        utilities::DateTimeBridge,
    },
};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub prefix: Option<String>,

    pub owner_id: Uuid,
    pub description: Option<String>,

    pub lead_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Project {
    pub async fn owner(&self, ctx: &Context<'_>) -> Result<Option<Member>> {
        let (_plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        Ok(loader.load_one(self.owner_id).await.unwrap())
    }

    pub async fn members(&self, ctx: &Context<'_>) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT member_id FROM members_by_projects
            WHERE project_id = $1
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

    pub async fn tasks(&self, ctx: &Context<'_>) -> Result<Vec<Task>> {
        //este caso específico necesita revisión
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let tasks = sqlx::query!(
            r#"
        SELECT * FROM tasks
        WHERE project_id = $1"#,
            &self.id
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(tasks
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
                parent_id: r.parent_id,
            })
            .collect())
    }

    pub async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let loader = ctx.data::<DataLoader<TeamLoader>>().unwrap();

        let ids: Vec<Uuid> = sqlx::query!(
            r#"
            SELECT team_id FROM teams_by_projects
            WHERE project_id = $1
            "#,
            &self.id
        )
        .fetch_all(&*plexo_engine.pool)
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

        Ok(teams.clone())
    }

    pub async fn leader(&self, ctx: &Context<'_>) -> Option<Member> {
        let loader = ctx.data::<DataLoader<MemberLoader>>().unwrap();

        //match to see is project_id is none
        match self.lead_id {
            Some(lead_id) => loader.load_one(lead_id).await.unwrap(),
            None => None,
        }
    }
}
