use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::{Member, MemberRole},
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub description: Option<String>,
    pub prefix: String,

    pub owner_id: Uuid,

}

#[ComplexObject]
impl Project {
    pub async fn owner(&self, ctx: &Context<'_>) -> Member {
        todo!()
    }

    pub async fn members(&self, ctx: &Context<'_>) -> Vec<Member> {
        todo!()
        //------falta crear tabla
        // let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        // let plexo_engine = ctx.data::<Engine>().unwrap();
        // let projects = sqlx::query!(r#"SELECT * FROM members_by_projects WHERE project_id = $1"#, &self.id).fetch_all(&plexo_engine.pool).await.unwrap();
        // projects
        //     .iter()
        //     .map(|r| Member {
        //         id: r.id,
        //         created_at: DateTimeBridge::from_offset_date_time(r.created_at),
        //         updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
        //         name: r.name.clone(),
        //         email: r.email.clone(),
        //         github_id: r.github_id.clone(),
        //         google_id: r.google_id.clone(),
        //         photo_url: r.photo_url.clone(),
        //         role: MemberRole::from_optional_str(&r.role),
        //     })
        //     .collect()    
    }

    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        todo!()
    }
}
