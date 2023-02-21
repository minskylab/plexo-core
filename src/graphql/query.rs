use async_graphql::{Context, InputObject, Object};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::{Member, MemberRole},
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

pub struct QueryRoot;

#[derive(InputObject)]
pub struct TaskFilter {
    pub project_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date_from: Option<DateTime<Utc>>,
    pub due_date_to: Option<DateTime<Utc>>,
}

#[derive(InputObject)]
pub struct MemberFilter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub github_id: Option<String>,
    pub role: Option<String>,
}

#[derive(InputObject)]
pub struct TeamFilter {
    pub visibility: Option<String>,
    pub name: Option<String>,
}

#[derive(InputObject)]
pub struct ProjectFilter {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[Object]
impl QueryRoot {
    async fn tasks(&self, ctx: &Context<'_>, filter: Option<TaskFilter>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let tasks = sqlx::query!(
            r#"
            SELECT * FROM tasks
            "#
        ).fetch_all(&plexo_engine.pool).await.unwrap();

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

    async fn task_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let task = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            id
        ).fetch_one(&plexo_engine.pool).await.unwrap();

        Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title.clone(),
            description: task.description.clone(),
            status: TaskStatus::from_optional_str(&task.status),
            priority: TaskPriority::from_optional_str(&task.priority),
            due_date: task
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task.project_id,
            lead_id: task.lead_id,
            labels: task
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
            owner_id: task.owner_id.unwrap_or(Uuid::nil()),
            count: task.count,
        }
    }

    async fn members(&self, ctx: &Context<'_>, filter: Option<MemberFilter>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let members = sqlx::query!(
            r#"
            SELECT * FROM members
            "#
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap();

        // let mut results = vec![];
        // for r in members.iter() {
        //     let mut member = Member {
        //         id: r.id,
        //         created_at: DateTimeBridge::from_offset_date_time(r.created_at),
        //         updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
        //         ...
        //         tasks: None,
        //     };
        //     let tasks = member.owned_tasks(ctx).await.unwrap();
        //     member.tasks = Some(tasks);
        //     results.push(member);
        // }
        // results

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

    async fn member_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            id
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

    async fn member_by_email(&self, ctx: &Context<'_>, email: String) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE email = $1
            "#,
            email
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

    async fn projects(&self, ctx: &Context<'_>, filter: Option<ProjectFilter>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let projects = sqlx::query!(
            r#"
            SELECT * FROM projects
            "#
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
                prefix: r.prefix.clone(),
                owner_id: r.owner_id.unwrap_or(Uuid::nil()),
                description: r.description.clone(),
                lead_id: r.lead_id.clone(),
                start_date: r
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
                due_date: r
                    .due_date
                    .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),

            })
            .collect()
    }

    async fn project_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let project = sqlx::query!(
            r#"
            SELECT * FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Project {
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

        }
    }

    async fn teams(&self, ctx: &Context<'_>, filter: Option<TeamFilter>) -> Vec<Team> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let teams = sqlx::query!(
            r#"
            SELECT *
            FROM teams
            "#
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
                prefix: r.prefix.clone().unwrap_or("".to_string()),

            })
            .collect()
    }

    async fn team_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name,
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.unwrap_or("".to_string()),
        }
    }
}
