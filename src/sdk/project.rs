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
    pub prefix: Option<String>,

    pub owner_id: Uuid,
    pub description: Option<String>,

    pub lead_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Project {
    pub async fn owner(&self, ctx: &Context<'_>) -> Option<Member> {
        //cambiado a Option, pq hay un id que no tiene user
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let member = sqlx::query!(r#"SELECT * FROM members WHERE id = $1"#, &self.owner_id)
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

    pub async fn members (&self, ctx: &Context<'_>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let members = sqlx::query!(
        r#"
        SELECT * FROM members_by_projects JOIN members
        ON members_by_projects.member_id = members.id WHERE project_id = $1"#,
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

    pub async fn tasks (&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let tasks = sqlx::query!(
        r#"
        SELECT * FROM tasks
        WHERE project_id = $1"#,
         &self.id)
         .fetch_all(&plexo_engine.pool).await.unwrap();

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
                count: r.count,
            })
            .collect()
    } 

    // pub async fn tasks (&self, ctx: &Context<'_>) -> Vec<Task> {
    //     let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
    //     let plexo_engine = ctx.data::<Engine>().unwrap();

    //     let tasks = sqlx::query!(
    //     r#"
    //     SELECT 
    //     tasks.id,
    //     tasks.created_at,
    //     tasks.updated_at,
    //     tasks.title,
    //     tasks.description,
    //     tasks.status,
    //     tasks.priority,
    //     tasks.due_date,
    //     tasks.project_id,
    //     tasks.lead_id,
    //     tasks.labels,
    //     tasks.owner_id,
    //     tasks.count
    //     FROM tasks_by_projects JOIN tasks 
    //     ON tasks_by_projects.task_id = tasks.id WHERE tasks_by_projects.project_id = $1
    //     "#,
    //     &self.id
    //     )
    //     .fetch_all(&plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     tasks
    //         .iter()
    //         .map(|r| Task {
    //             id: r.id,
    //             created_at: DateTimeBridge::from_offset_date_time(r.created_at),
    //             updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
    //             title: r.title.clone(),
    //             description: r.description.clone(),
    //             status: TaskStatus::from_optional_str(&r.status),
    //             priority: TaskPriority::from_optional_str(&r.priority),
    //             due_date: r.due_date.map(|d| DateTimeBridge::from_offset_date_time(d)),
    //             project_id: r.project_id,
    //             lead_id: r.lead_id,
    //             labels: r
    //                 .labels
    //                 .as_ref()
    //                 .map(|l| {
    //                     l.as_array()
    //                         .unwrap()
    //                         .iter()
    //                         .map(|s| s.as_str().unwrap().to_string())
    //                         .collect()
    //                 })
    //                 .unwrap_or(vec![]),
    //             owner_id: r.owner_id.unwrap_or(Uuid::nil()),
    //             count: r.count,
    //         })
    //         .collect()
    // }
 pub async fn teams (&self, ctx: &Context<'_>) -> Vec<Team> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let teams = sqlx::query!(
        r#"
        SELECT * FROM teams_by_projects JOIN teams
        ON teams_by_projects.team_id = teams.id WHERE project_id = $1"#,
         &self.id)
         .fetch_all(&plexo_engine.pool).await.unwrap();

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
