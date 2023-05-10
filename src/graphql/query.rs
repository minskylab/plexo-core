use std::str::FromStr;

use async_graphql::{Context, InputObject, Object};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    llm::suggestions::{TaskSuggestion, TaskSuggestionResult},
    sdk::{
        labels::Label,
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
    async fn tasks(&self, ctx: &Context<'_>, _filter: Option<TaskFilter>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap();
        let plexo_engine = ctx.data::<Engine>().unwrap();

        plexo_engine
            .auth
            .extract_claims_from_access_token(auth_token)
            .await;

        let tasks = sqlx::query!(
            r#"
            SELECT * FROM tasks
            "#
        )
        .fetch_all(&*plexo_engine.pool)
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
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title.clone(),
            description: task.description.clone(),
            status: TaskStatus::from_optional_str(&task.status),
            priority: TaskPriority::from_optional_str(&task.priority),
            due_date: task.due_date.map(DateTimeBridge::from_offset_date_time),
            project_id: task.project_id,
            lead_id: task.lead_id,
            owner_id: task.owner_id,
            count: task.count,
        }
    }

    async fn members(&self, ctx: &Context<'_>, _filter: Option<MemberFilter>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let members = sqlx::query!(
            r#"
            SELECT * FROM members
            "#
        )
        .fetch_all(&*plexo_engine.pool)
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
        .fetch_one(&*plexo_engine.pool)
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
        .fetch_one(&*plexo_engine.pool)
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

    async fn projects(&self, ctx: &Context<'_>, _filter: Option<ProjectFilter>) -> Vec<Project> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let projects = sqlx::query!(
            r#"
            SELECT * FROM projects
            "#
        )
        .fetch_all(&*plexo_engine.pool)
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
                owner_id: r.owner_id,
                description: r.description.clone(),
                lead_id: r.lead_id,
                start_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
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
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Project {
            id: project.id,
            created_at: DateTimeBridge::from_offset_date_time(project.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
            name: project.name.clone(),
            description: project.description.clone(),
            prefix: project.prefix.clone(),
            owner_id: project.owner_id,
            lead_id: project.lead_id,
            start_date: project.due_date.map(DateTimeBridge::from_offset_date_time),
            due_date: project.due_date.map(DateTimeBridge::from_offset_date_time),
        }
    }

    async fn teams(&self, ctx: &Context<'_>, _filter: Option<TeamFilter>) -> Vec<Team> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let teams = sqlx::query!(
            r#"
            SELECT *
            FROM teams
            "#
        )
        .fetch_all(&*plexo_engine.pool)
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
                prefix: r.prefix.clone(),
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
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name,
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix,
        }
    }

    async fn labels(&self, ctx: &Context<'_>) -> Vec<Label> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let labels = sqlx::query!(
            r#"
            SELECT * FROM labels
            "#
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        labels
            .iter()
            .map(|r| Label {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                color: r.color.clone(),
                description: r.description.clone(),
            })
            .collect()
    }

    async fn suggest_new_task(
        &self,
        ctx: &Context<'_>,
        task: TaskSuggestion,
    ) -> TaskSuggestionResult {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        println!("token: {}", auth_token);

        let raw_suggestion = plexo_engine
            .auto_suggestions_engine
            .get_suggestions(task)
            .await;

        let parts = raw_suggestion
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let title = parts[0].replace("Task Title:", "").trim().to_string();
        let description = parts[1].replace("Task Description:", "").trim().to_string();
        let status = parts[2].replace("Task Status:", "").trim().to_string();
        let priority = parts[3].replace("Task Priority:", "").trim().to_string();
        let due_date = parts[4].replace("Task Due Date:", "").trim().to_string();

        // TODO: parse raw_suggestion to TaskSuggestion

        // Example of response:
        // "Task Title: Boostrap github project\nTask Description: Bootstrap a new Github project with necessary codebase and documentation.\nTask Status: InProgress\nTask Priority: High\nTask Due Date: 2023-04-28T05:00:00+00:00"

        let status = TaskStatus::from_str(&status).unwrap();
        let priority = TaskPriority::from_str(&priority).unwrap();
        let due_date = DateTime::<Utc>::from_str(&due_date).unwrap();

        TaskSuggestionResult {
            title,
            description,
            status,
            priority,
            due_date,
        }
    }
}
