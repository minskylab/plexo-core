use std::str::FromStr;

use async_graphql::{Context, InputObject, Object, Result};
use async_graphql::types::connection::*;

use chrono::{DateTime, Utc, NaiveDateTime};
use uuid::Uuid;

use crate::{
    graphql::auth::extract_context,
    sdk::{
        activity::{Activity, ActivityOperationType, ActivityResourceType},
        labels::Label,
        member::{Member, MemberRole},
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
        utilities::DateTimeBridge,
    },
};

// use super::auth::extract_context;

#[derive(Default)]
pub struct ResourcesQuery;

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
impl ResourcesQuery {
    async fn tasks_pag(&self,
        ctx: &Context<'_>,
        _filter: Option<TaskFilter>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<DateTime<Utc>, Task, EmptyFields, EmptyFields>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let mut start = 0;
        let mut limit = 100;

        if let Some(first) = first {
        limit = first as usize;
        }

        if let Some(after) = &after {
        start = after.parse().unwrap_or(0);
        }

        let tasks = sqlx::query!(
        r#"
        SELECT * FROM tasks ORDER BY created_at ASC LIMIT $1 OFFSET $2
        "#,
        limit as i32, 
        start as i32
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        let items: Vec<_> = tasks.iter().map(|r| Task {
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
        }).collect();

        let mut connection = Connection::with_additional_fields(false, false, EmptyFields);

        connection.edges.extend(
        items.iter().enumerate().map(|(_index, task)| {
            Edge::with_additional_fields(task.created_at, task.clone(), EmptyFields)
        })
        );

        Ok(connection)
    }
    
    async fn tasks(&self, ctx: &Context<'_>, _filter: Option<TaskFilter>) -> Result<Vec<Task>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let tasks = sqlx::query!(
            r#"
            SELECT * FROM tasks
            "#
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

    async fn task_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

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

        Ok(Task {
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
            parent_id: task.parent_id,
        })
    }

    async fn members(
        &self,
        ctx: &Context<'_>,
        _filter: Option<MemberFilter>,
    ) -> Result<Vec<Member>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let members = sqlx::query!(
            r#"
            SELECT * FROM members
            "#
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(members
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
                password_hash: None,
            })
            .collect())
    }

    async fn member_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

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

        Ok(Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name.clone(),
            email: member.email.clone(),
            github_id: member.github_id.clone(),
            google_id: member.google_id.clone(),
            photo_url: member.photo_url.clone(),
            role: MemberRole::from_optional_str(&member.role),
            password_hash: None,
        })
    }

    async fn member_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

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

        Ok(Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name.clone(),
            email: member.email.clone(),
            github_id: member.github_id.clone(),
            google_id: member.google_id.clone(),
            photo_url: member.photo_url.clone(),
            role: MemberRole::from_optional_str(&member.role),
            password_hash: None,
        })
    }

    async fn projects(
        &self,
        ctx: &Context<'_>,
        _filter: Option<ProjectFilter>,
    ) -> Result<Vec<Project>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let projects = sqlx::query!(
            r#"
            SELECT * FROM projects
            "#
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(projects
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
                start_date: r.start_date.map(DateTimeBridge::from_offset_date_time),
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
            })
            .collect())
    }

    async fn project_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

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

        Ok(Project {
            id: project.id,
            created_at: DateTimeBridge::from_offset_date_time(project.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
            name: project.name.clone(),
            description: project.description.clone(),
            prefix: project.prefix.clone(),
            owner_id: project.owner_id,
            lead_id: project.lead_id,
            start_date: project
                .start_date
                .map(DateTimeBridge::from_offset_date_time),
            due_date: project.due_date.map(DateTimeBridge::from_offset_date_time),
        })
    }

    async fn teams(&self, ctx: &Context<'_>, _filter: Option<TeamFilter>) -> Result<Vec<Team>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let teams = sqlx::query!(
            r#"
            SELECT *
            FROM teams
            "#
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(teams
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
            .collect())
    }

    async fn team_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

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

        Ok(Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name,
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix,
        })
    }

    async fn labels(&self, ctx: &Context<'_>) -> Result<Vec<Label>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let labels = sqlx::query!(
            r#"
            SELECT * FROM labels
            "#
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(labels
            .iter()
            .map(|r| Label {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                color: r.color.clone(),
                description: r.description.clone(),
            })
            .collect())
    }

    async fn me(&self, ctx: &Context<'_>) -> Result<Member> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        let r = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(Member {
            id: r.id,
            created_at: DateTimeBridge::from_offset_date_time(r.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
            name: r.name.clone(),
            email: r.email.clone(),
            github_id: r.github_id.clone(),
            google_id: r.google_id.clone(),
            photo_url: r.photo_url.clone(),
            role: MemberRole::from_optional_str(&r.role),
            password_hash: None,
        })
    }

    async fn activity(
        &self,
        ctx: &Context<'_>,
        resource_type: Option<ActivityResourceType>,
        resource_id: Option<Uuid>,
        operation_type: Option<ActivityOperationType>,
        member_id: Option<Uuid>,
    ) -> Result<Vec<Activity>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let activities = sqlx::query!(
            r#"
            SELECT * FROM activity
            WHERE
                resource_type = COALESCE($1, resource_type)
                AND resource_id = COALESCE($2, resource_id)
                AND operation = COALESCE($3, operation)
                AND member_id = COALESCE($4, member_id)
            "#,
            resource_type.map(|r| r.to_string()),
            resource_id,
            operation_type.map(|r| r.to_string()),
            member_id
        )
        .fetch_all(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(activities
            .iter()
            .map(|r| Activity {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                resource_type: ActivityResourceType::from_str(&r.resource_type).unwrap(),
                operation: ActivityOperationType::from_str(&r.operation).unwrap(),
                resource_id: r.resource_id,
                member_id: r.member_id,
            })
            .collect())
    }
}
