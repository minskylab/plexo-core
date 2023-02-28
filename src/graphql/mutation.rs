use std::process::id;

use async_graphql::{ComplexObject, Context, InputObject, InputType, Object};
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::PgRow, query, types::time::OffsetDateTime, types::time::PrimitiveDateTime, Pool,
    Postgres, Row,
};
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

#[derive(InputObject)]
struct AssigneesOperation {
    _append: Option<Vec<Uuid>>,
    _delete: Option<Vec<Uuid>>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: Option<String>,
        owner_id: Uuid,
        status: Option<String>,
        priority: Option<String>,
        due_date: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        lead_id: Option<Uuid>,
        labels: Option<Vec<String>>,
        add_assignees_id: Option<Vec<Uuid>>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let task_create = sqlx::query!(
            r#"
            INSERT INTO tasks
            (title, owner_id)
            VALUES ($1, $2)
            RETURNING id
            "#,
            title,
            owner_id
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        if description.is_some() {
            let _task_update_description = sqlx::query!(
                r#"
                UPDATE tasks
                SET description = $1
                WHERE id = $2
                "#,
                description.unwrap(),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if status.is_some() {
            let _task_update_status = sqlx::query!(
                r#"
                UPDATE tasks
                SET status = $1
                WHERE id = $2
                "#,
                status.unwrap(),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if priority.is_some() {
            let _task_update_priority = sqlx::query!(
                r#"
                UPDATE tasks
                SET priority = $1
                WHERE id = $2
                "#,
                priority.unwrap(),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if due_date.is_some() {
            let _task_update_due_date = sqlx::query!(
                r#"
                UPDATE tasks
                SET due_date = $1
                WHERE id = $2
                "#,
                due_date.map(|d| DateTimeBridge::from_date_time(d)),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if project_id.is_some() {
            let _task_update_project_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET project_id = $1
                WHERE id = $2
                "#,
                project_id.unwrap(),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if lead_id.is_some() {
            let _task_update_lead_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET lead_id = $1
                WHERE id = $2
                "#,
                lead_id.unwrap(),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if labels.is_some() {
            let _task_update_labels = sqlx::query!(
                r#"
                UPDATE tasks
                SET labels = $1
                WHERE id = $2
                "#,
                labels.map(|l| serde_json::to_value(l).unwrap()),
                task_create.id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_assignees_id.is_some() {
            self.add_assignees_to_task(ctx, task_create.id, add_assignees_id.unwrap())
                .await;
        }

        let task_final_info = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            task_create.id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        let task = Task {
            id: task_final_info.id,
            created_at: DateTimeBridge::from_offset_date_time(task_final_info.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task_final_info.updated_at),
            title: task_final_info.title,
            description: task_final_info.description,
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            labels: task_final_info
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
            owner_id: task_final_info.owner_id.unwrap_or(Uuid::nil()),
            count: task_final_info.count,
        };

        // plexo_engine
        //     .subscription_manager
        //     .broadcast_task_created(auth_token, task)
        //     .await;

        task
    }

    async fn update_task(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
        status: Option<String>,
        priority: Option<String>,
        due_date: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        lead_id: Option<Uuid>,
        labels: Option<Vec<String>>,

        assignees: Option<AssigneesOperation>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        if title.is_some() {
            let _task_update_description = sqlx::query!(
                r#"
                UPDATE tasks
                SET title = $1
                WHERE id = $2
                "#,
                title.unwrap().clone(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if description.is_some() {
            let _task_update_description = sqlx::query!(
                r#"
                UPDATE tasks
                SET description = $1
                WHERE id = $2
                "#,
                description.unwrap().clone(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if status.is_some() {
            let _task_update_status = sqlx::query!(
                r#"
                UPDATE tasks
                SET status = $1
                WHERE id = $2
                "#,
                status.unwrap().clone(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if priority.is_some() {
            let _task_update_priority = sqlx::query!(
                r#"
                UPDATE tasks
                SET priority = $1
                WHERE id = $2
                "#,
                priority.unwrap().clone(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if due_date.is_some() {
            let _task_update_due_date = sqlx::query!(
                r#"
                UPDATE tasks
                SET due_date = $1
                WHERE id = $2
                "#,
                due_date.map(|d| DateTimeBridge::from_date_time(d)),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if project_id.is_some() {
            let _task_update_project_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET project_id = $1
                WHERE id = $2
                "#,
                project_id.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if lead_id.is_some() {
            let _task_update_lead_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET lead_id = $1
                WHERE id = $2
                "#,
                lead_id.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if labels.is_some() {
            let _task_update_labels = sqlx::query!(
                r#"
                UPDATE tasks
                SET labels = $1
                WHERE id = $2
                "#,
                labels.map(|l| serde_json::to_value(l).unwrap()),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let a = match assignees {
            Some(operation) => match operation {
                AssigneesOperation {
                    _append: None,
                    _delete: None,
                } => (),
                AssigneesOperation {
                    _append,
                    _delete: None,
                } => {
                    self.add_assignees_to_task(ctx, id, _append.unwrap())
                        .await
                        .unwrap();
                }
                AssigneesOperation {
                    _delete,
                    _append: None,
                } => {
                    self.delete_assignees_from_task(ctx, id, _delete.unwrap())
                        .await
                        .unwrap();
                }
                AssigneesOperation { _append, _delete } => {
                    self.add_assignees_to_task(ctx, id, _append.unwrap())
                        .await
                        .unwrap();
                    self.delete_assignees_from_task(ctx, id, _delete.unwrap())
                        .await
                        .unwrap();
                }
            },
            None => (),
        };

        let task_final_info = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            id.clone(),
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Task {
            id: id,
            created_at: DateTimeBridge::from_offset_date_time(task_final_info.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task_final_info.updated_at),
            title: task_final_info.title.clone(),
            description: task_final_info.description.clone(),
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            labels: task_final_info
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
            owner_id: task_final_info.owner_id.unwrap_or(Uuid::nil()),
            count: task_final_info.count,
        }
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let task = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            RETURNING id, created_at, updated_at, title, description, owner_id, status, priority, due_date, project_id, lead_id, labels, count
            "#,
            id,
        )
        .fetch_one(&plexo_engine.pool).await.unwrap();

        let _deleted_assignees = sqlx::query!(
            r#"
            DELETE FROM tasks_by_assignees
            WHERE task_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
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

    // async fn create_member(
    //     &self,
    //     email: String,
    //     password: String,
    //     first_name: String,
    //     last_name: String,
    // ) -> Member {
    //     todo!()
    // }

    async fn update_member(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        email: Option<String>,
        name: Option<String>,
        add_projects_id: Option<Vec<Uuid>>,
        delete_projects_id: Option<Vec<Uuid>>,
        add_teams_id: Option<Vec<Uuid>>,
        delete_teams_id: Option<Vec<Uuid>>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        if email.is_some() {
            let _member_update_email = sqlx::query!(
                r#"
                UPDATE members
                SET email = $1
                WHERE id = $2
                "#,
                email.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if name.is_some() {
            let _member_update_name = sqlx::query!(
                r#"
                UPDATE members
                SET name = $1
                WHERE id = $2
                "#,
                name.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_projects_id.is_some() {
            self.add_projects_to_member(ctx, id.clone(), add_projects_id.unwrap())
                .await;
        }

        if delete_projects_id.is_some() {
            self.delete_projects_from_member(ctx, id.clone(), delete_projects_id.unwrap())
                .await;
        }

        if add_teams_id.is_some() {
            self.add_teams_to_member(ctx, id.clone(), add_teams_id.unwrap())
                .await;
        }

        if delete_teams_id.is_some() {
            self.delete_teams_from_member(ctx, id.clone(), delete_teams_id.unwrap())
                .await;
        }

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            id.clone(),
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
            github_id: member.github_id,
            google_id: member.google_id,
            photo_url: member.photo_url,
            role: MemberRole::from_optional_str(&member.role),
        }
    }

    async fn delete_member(&self, ctx: &Context<'_>, id: Uuid) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let member = sqlx::query!(
            r#"
            DELETE FROM members
            WHERE id = $1
            RETURNING id, created_at, updated_at, name, email, github_id, google_id, photo_url, role;
            "#,
            id,
        )
        .fetch_one(&plexo_engine.pool).await.unwrap();

        let _deleted_projects = sqlx::query!(
            r#"
            DELETE FROM members_by_projects
            WHERE member_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_teams = sqlx::query!(
            r#"
            DELETE FROM members_by_teams
            WHERE member_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_tasks = sqlx::query!(
            r#"
            DELETE FROM tasks_by_assignees
            WHERE assignee_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name.clone(),
            email: member.email.clone(),
            github_id: member.github_id,
            google_id: member.google_id,
            photo_url: member.photo_url,
            role: MemberRole::from_optional_str(&member.role),
        }
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
        prefix: Option<String>,
        owner_id: Uuid,
        description: Option<String>,
        lead_id: Option<Uuid>,
        start_date: Option<DateTime<Utc>>,
        due_date: Option<DateTime<Utc>>,
        add_members_id: Option<Vec<Uuid>>,
        add_teams_id: Option<Vec<Uuid>>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let project_create = sqlx::query!(
            r#"
            INSERT INTO projects (name, owner_id)
            VALUES ($1, $2)
            RETURNING id
            "#,
            name,
            owner_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        if prefix.is_some() {
            let _project_update_prefix = sqlx::query!(
                r#"
                UPDATE projects
                SET prefix = $1
                WHERE id = $2
                "#,
                prefix.unwrap(),
                project_create.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if description.is_some() {
            let _project_update_description = sqlx::query!(
                r#"
                UPDATE projects
                SET description = $1
                WHERE id = $2
                "#,
                description.unwrap(),
                project_create.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if lead_id.is_some() {
            let _project_update_lead_id = sqlx::query!(
                r#"
                UPDATE projects
                SET lead_id = $1
                WHERE id = $2
                "#,
                lead_id.unwrap(),
                project_create.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if start_date.is_some() {
            let _project_update_start_date = sqlx::query!(
                r#"
                UPDATE projects
                SET start_date = $1
                WHERE id = $2
                "#,
                DateTimeBridge::from_primitive_to_date_time(start_date.unwrap()),
                project_create.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if due_date.is_some() {
            let _project_update_due_date = sqlx::query!(
                r#"
                UPDATE projects
                SET due_date = $1
                WHERE id = $2
                "#,
                DateTimeBridge::from_primitive_to_date_time(due_date.unwrap()),
                project_create.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_members_id.is_some() {
            self.add_members_to_project(ctx, project_create.id.clone(), add_members_id.unwrap())
                .await;
        }

        if add_teams_id.is_some() {
            self.add_teams_to_project(ctx, project_create.id.clone(), add_teams_id.unwrap())
                .await;
        }

        let project = sqlx::query!(
            r#"
            SELECT *
            FROM projects
            WHERE id = $1
            "#,
            project_create.id.clone(),
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

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        prefix: Option<String>,
        owner_id: Option<Uuid>,
        description: Option<String>,
        lead_id: Option<Uuid>,
        start_date: Option<DateTime<Utc>>,
        due_date: Option<DateTime<Utc>>,
        add_members_id: Option<Vec<Uuid>>,
        delete_members_id: Option<Vec<Uuid>>,
        add_teams_id: Option<Vec<Uuid>>,
        delete_teams_id: Option<Vec<Uuid>>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        if name.is_some() {
            let _project_update_name = sqlx::query!(
                r#"
                UPDATE projects
                SET name = $1
                WHERE id = $2
                "#,
                name.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if prefix.is_some() {
            let _project_update_prefix = sqlx::query!(
                r#"
                UPDATE projects
                SET prefix = $1
                WHERE id = $2
                "#,
                prefix.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if owner_id.is_some() {
            let _project_update_owner_id = sqlx::query!(
                r#"
                UPDATE projects
                SET owner_id = $1
                WHERE id = $2
                "#,
                owner_id.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if description.is_some() {
            let _project_update_description = sqlx::query!(
                r#"
                UPDATE projects
                SET description = $1
                WHERE id = $2
                "#,
                description.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if lead_id.is_some() {
            let _project_update_lead_id = sqlx::query!(
                r#"
                UPDATE projects
                SET lead_id = $1
                WHERE id = $2
                "#,
                lead_id.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if start_date.is_some() {
            let _project_update_start_date = sqlx::query!(
                r#"
                UPDATE projects
                SET start_date = $1
                WHERE id = $2
                "#,
                DateTimeBridge::from_primitive_to_date_time(start_date.unwrap()),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if due_date.is_some() {
            let _project_update_due_date = sqlx::query!(
                r#"
                UPDATE projects
                SET due_date = $1
                WHERE id = $2
                "#,
                DateTimeBridge::from_primitive_to_date_time(due_date.unwrap()),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_members_id.is_some() {
            self.add_members_to_project(ctx, id.clone(), add_members_id.unwrap())
                .await;
        }

        if delete_members_id.is_some() {
            self.delete_members_from_project(ctx, id.clone(), delete_members_id.unwrap())
                .await;
        }

        if add_teams_id.is_some() {
            self.add_teams_to_project(ctx, id.clone(), add_teams_id.unwrap())
                .await;
        }

        if delete_teams_id.is_some() {
            self.delete_teams_from_project(ctx, id.clone(), delete_teams_id.unwrap())
                .await;
        }

        let project = sqlx::query!(
            r#"
            SELECT *
            FROM projects
            WHERE id = $1
            "#,
            id.clone(),
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

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let project = sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_members = sqlx::query!(
            r#"
            DELETE FROM members_by_projects
            WHERE project_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_teams = sqlx::query!(
            r#"
            DELETE FROM teams_by_projects
            WHERE project_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_tasks = sqlx::query!(
            r#"
            UPDATE tasks
            SET project_id = NULL
            WHERE project_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
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

    async fn create_team(
        &self,
        ctx: &Context<'_>,
        name: String,
        owner_id: Uuid,
        visibility: Option<String>,
        prefix: Option<String>,
        add_members_id: Option<Vec<Uuid>>,
        add_projects_id: Option<Vec<Uuid>>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let _create_team = sqlx::query!(
            r#"
            INSERT INTO teams (name, owner_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            name,
            owner_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        if visibility.is_some() {
            let _team_update_visibility = sqlx::query!(
                r#"
                UPDATE teams
                SET visibility = $1
                WHERE id = $2
                "#,
                visibility.clone().unwrap(),
                _create_team.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if prefix.is_some() {
            let _team_update_prefix = sqlx::query!(
                r#"
                UPDATE teams
                SET prefix = $1
                WHERE id = $2
                "#,
                prefix.unwrap(),
                _create_team.id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_members_id.is_some() {
            self.add_members_to_team(ctx, _create_team.id.clone(), add_members_id.unwrap())
                .await;
        }

        if add_projects_id.is_some() {
            self.add_projects_to_team(ctx, _create_team.id.clone(), add_projects_id.unwrap())
                .await;
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            _create_team.id.clone(),
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        }
    }

    async fn update_team(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        owner_id: Option<Uuid>,
        visibility: Option<String>,
        prefix: Option<String>,
        add_members_id: Option<Vec<Uuid>>,
        delete_members_id: Option<Vec<Uuid>>,
        add_projects_id: Option<Vec<Uuid>>,
        delete_projects_id: Option<Vec<Uuid>>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        if name.is_some() {
            let _team_update_name = sqlx::query!(
                r#"
                UPDATE teams
                SET name = $1
                WHERE id = $2
                "#,
                name.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if owner_id.is_some() {
            let _team_update_owner_id = sqlx::query!(
                r#"
                UPDATE teams
                SET owner_id = $1
                WHERE id = $2
                "#,
                owner_id.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if visibility.is_some() {
            let _team_update_visibility = sqlx::query!(
                r#"
                UPDATE teams
                SET visibility = $1
                WHERE id = $2
                "#,
                visibility.clone().unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if prefix.is_some() {
            let _team_update_prefix = sqlx::query!(
                r#"
                UPDATE teams
                SET prefix = $1
                WHERE id = $2
                "#,
                prefix.unwrap(),
                id.clone(),
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        if add_members_id.is_some() {
            self.add_members_to_team(ctx, id.clone(), add_members_id.unwrap())
                .await;
        }

        if delete_members_id.is_some() {
            self.delete_members_from_team(ctx, id.clone(), delete_members_id.unwrap())
                .await;
        }

        if add_projects_id.is_some() {
            self.add_projects_to_team(ctx, id.clone(), add_projects_id.unwrap())
                .await;
        }

        if delete_projects_id.is_some() {
            self.delete_projects_from_team(ctx, id.clone(), delete_projects_id.unwrap())
                .await;
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            id.clone(),
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        }
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: Uuid) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let team = sqlx::query!(
            r#"
            DELETE FROM teams
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        let _delete_team_members = sqlx::query!(
            r#"
            DELETE FROM members_by_teams
            WHERE team_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        let _delete_team_projects = sqlx::query!(
            r#"
            DELETE FROM teams_by_projects
            WHERE team_id = $1
            "#,
            id,
        )
        .execute(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        }
    }

    async fn add_assignees_to_task(
        &self,
        ctx: &Context<'_>,
        task_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _task_assign_member = sqlx::query!(
                r#"
                INSERT INTO tasks_by_assignees (task_id, assignee_id)
                VALUES ($1, $2)
                "#,
                task_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let task = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            task_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title,
            description: task.description,
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

    async fn delete_assignees_from_task(
        &self,
        ctx: &Context<'_>,
        task_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _task_assign_member = sqlx::query!(
                r#"
            DELETE FROM tasks_by_assignees
            WHERE task_id = $1 AND assignee_id = $2
            "#,
                task_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let task = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            task_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title,
            description: task.description,
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

    async fn add_projects_to_member(
        &self,
        ctx: &Context<'_>,
        member_id: Uuid,
        projects_id: Vec<Uuid>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for project_id in projects_id {
            let _member_assign_project = sqlx::query!(
                r#"
            INSERT INTO members_by_projects (project_id, member_id)
            VALUES ($1, $2)
            "#,
                project_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id,
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

    async fn delete_projects_from_member(
        &self,
        ctx: &Context<'_>,
        member_id: Uuid,
        projects_id: Vec<Uuid>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for project_id in projects_id {
            let _member_assign_project = sqlx::query!(
                r#"
            DELETE FROM members_by_projects
            WHERE project_id = $1 AND member_id = $2
            "#,
                project_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id,
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

    async fn add_members_to_project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _project_assign_member = sqlx::query!(
                r#"
            INSERT INTO members_by_projects (project_id, member_id)
            VALUES ($1, $2)
            "#,
                project_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let project = sqlx::query!(
            r#"
            SELECT * FROM projects
            WHERE id = $1
            "#,
            project_id,
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

    async fn delete_members_from_project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _project_assign_member = sqlx::query!(
                r#"
            DELETE FROM members_by_projects
            WHERE project_id = $1 AND member_id = $2
            "#,
                project_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let project = sqlx::query!(
            r#"
            SELECT * FROM projects
            WHERE id = $1
            "#,
            project_id,
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

    async fn add_members_to_team(
        &self,
        ctx: &Context<'_>,
        team_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _team_assign_member = sqlx::query!(
                r#"
            INSERT INTO members_by_teams (team_id, member_id)
            VALUES ($1, $2)
            "#,
                team_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            team_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        }
    }

    async fn delete_members_from_team(
        &self,
        ctx: &Context<'_>,
        team_id: Uuid,
        members_id: Vec<Uuid>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for member_id in members_id {
            let _team_assign_member = sqlx::query!(
                r#"
            DELETE FROM members_by_teams
            WHERE team_id = $1 AND member_id = $2
            "#,
                team_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            team_id,
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        }
    }

    async fn add_teams_to_member(
        &self,
        ctx: &Context<'_>,
        member_id: Uuid,
        teams_id: Vec<Uuid>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for team_id in teams_id {
            let _member_assign_team = sqlx::query!(
                r#"
            INSERT INTO members_by_teams (team_id, member_id)
            VALUES ($1, $2)
            "#,
                team_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id,
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

    async fn delete_teams_from_member(
        &self,
        ctx: &Context<'_>,
        member_id: Uuid,
        teams_id: Vec<Uuid>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for team_id in teams_id {
            let _member_assign_team = sqlx::query!(
                r#"
            DELETE FROM members_by_teams
            WHERE team_id = $1 AND member_id = $2
            "#,
                team_id,
                member_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id,
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
    // async fn add_task_to_project (
    //     &self,
    //     ctx: &Context<'_>,
    //     project_id: Uuid,
    //     task_id: Uuid,
    // ) -> Project {
    //     let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
    //     let plexo_engine = ctx.data::<Engine>().unwrap();

    //     let _project_assign_task = sqlx::query!(
    //         r#"
    //         INSERT INTO tasks_by_projects (project_id, task_id)
    //         VALUES ($1, $2)
    //         "#,
    //         project_id,
    //         task_id,
    //     )
    //     .execute(&plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     let project = sqlx::query!(
    //         r#"
    //         SELECT * FROM projects
    //         WHERE id = $1
    //         "#,
    //         project_id,
    //     )
    //     .fetch_one(&plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     Project {
    //         id: project.id,
    //         created_at: DateTimeBridge::from_offset_date_time(project.created_at),
    //         updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
    //         name: project.name.clone(),
    //         description: project.description.clone(),
    //         prefix: project.prefix.clone(),
    //         owner_id: project.owner_id.unwrap_or(Uuid::nil()),
    //         lead_id: project.lead_id,
    //         start_date: project
    //             .due_date
    //             .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
    //         due_date: project
    //             .due_date
    //             .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),

    //     }

    // }

    // async fn delete_task_from_project (
    //     &self,
    //     ctx: &Context<'_>,
    //     project_id: Uuid,
    //     task_id: Uuid,
    // ) -> Project {
    //     let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
    //     let plexo_engine = ctx.data::<Engine>().unwrap();

    //     let _project_assign_task = sqlx::query!(
    //         r#"
    //         DELETE FROM tasks_by_projects
    //         WHERE project_id = $1 AND task_id = $2
    //         "#,
    //         project_id,
    //         task_id,
    //     )
    //     .execute(&plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     let project = sqlx::query!(
    //         r#"
    //         SELECT * FROM projects
    //         WHERE id = $1
    //         "#,
    //         project_id,
    //     )
    //     .fetch_one(&plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     Project {
    //         id: project.id,
    //         created_at: DateTimeBridge::from_offset_date_time(project.created_at),
    //         updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
    //         name: project.name.clone(),
    //         description: project.description.clone(),
    //         prefix: project.prefix.clone(),
    //         owner_id: project.owner_id.unwrap_or(Uuid::nil()),
    //         lead_id: project.lead_id,
    //         start_date: project
    //             .due_date
    //             .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),
    //         due_date: project
    //             .due_date
    //             .map(|d| DateTimeBridge::from_offset_date_time(d.assume_utc())),

    //     }

    // }

    async fn add_teams_to_project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        teams_id: Vec<Uuid>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for team_id in teams_id {
            let _project_assign_team = sqlx::query!(
                r#"
                INSERT INTO teams_by_projects (project_id, team_id)
                VALUES ($1, $2)
                "#,
                project_id,
                team_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let project = sqlx::query!(
            r#"
            SELECT * FROM projects
            WHERE id = $1
            "#,
            project_id,
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

    async fn delete_teams_from_project(
        &self,
        ctx: &Context<'_>,
        project_id: Uuid,
        teams_id: Vec<Uuid>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for team_id in teams_id {
            let _project_assign_team = sqlx::query!(
                r#"
                DELETE FROM teams_by_projects
                WHERE project_id = $1 AND team_id = $2
                "#,
                project_id,
                team_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let project = sqlx::query!(
            r#"
            SELECT * FROM projects
            WHERE id = $1
            "#,
            project_id,
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

    async fn add_projects_to_team(
        &self,
        ctx: &Context<'_>,
        team_id: Uuid,
        projects_id: Vec<Uuid>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for project_id in projects_id {
            let _project_assign_team = sqlx::query!(
                r#"
                INSERT INTO teams_by_projects (project_id, team_id)
                VALUES ($1, $2)
                "#,
                project_id,
                team_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            team_id,
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
            prefix: team.prefix,
        }
    }

    async fn delete_projects_from_team(
        &self,
        ctx: &Context<'_>,
        team_id: Uuid,
        projects_id: Vec<Uuid>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        for project_id in projects_id {
            let _project_assign_team = sqlx::query!(
                r#"
                DELETE FROM teams_by_projects
                WHERE project_id = $1 AND team_id = $2
                "#,
                project_id,
                team_id,
            )
            .execute(&plexo_engine.pool)
            .await
            .unwrap();
        }

        let team = sqlx::query!(
            r#"
            SELECT * FROM teams
            WHERE id = $1
            "#,
            team_id,
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
            prefix: team.prefix,
        }
    }
}
