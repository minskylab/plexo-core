use std::process::id;

use async_graphql::{Context, InputType, Object, ComplexObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::{Pool, Postgres, query, types::time::OffsetDateTime, Row, postgres::PgRow };

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::{Member, MemberRole},
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

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
        assignee_id: Option<Uuid>,
        labels: Option<Vec<String>>,
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
            
            ).fetch_one(&plexo_engine.pool).await.unwrap();
        
        if description.is_some() {

            let _task_update_description = sqlx::query!(
                r#"
                UPDATE tasks
                SET description = $1
                WHERE id = $2
                "#,
                description.unwrap(),
                task_create.id,
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        if assignee_id.is_some() {
            let _task_update_assignee_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET assignee_id = $1
                WHERE id = $2
                "#,
                assignee_id.unwrap(),
                task_create.id,
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        let task_final_info = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, title, description, status, priority, due_date, project_id, assignee_id, labels, owner_id
            FROM tasks
            WHERE id = $1
            "#,
            task_create.id,
        ).fetch_one(&plexo_engine.pool).await.unwrap();


        Task {
            id: task_final_info.id,
            created_at: DateTimeBridge::from_offset_date_time(task_final_info.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task_final_info.updated_at),
            title: task_final_info.title,
            description: task_final_info.description,
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info.due_date.map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task_final_info.project_id,
            assignee_id: task_final_info.assignee_id,
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
            
        }
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
        assignee_id: Option<Uuid>,
        labels: Option<Vec<String>>,
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        if assignee_id.is_some() {
            let _task_update_assignee_id = sqlx::query!(
                r#"
                UPDATE tasks
                SET assignee_id = $1
                WHERE id = $2
                "#,
                assignee_id.unwrap(),
                id.clone(),
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        let task_final_info = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, title, description, status, priority, due_date, project_id, assignee_id, labels, owner_id
            FROM tasks
            WHERE id = $1
            "#,
            id.clone(),
        ).fetch_one(&plexo_engine.pool).await.unwrap();


        Task {
            id: id,
            created_at: DateTimeBridge::from_offset_date_time(task_final_info.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task_final_info.updated_at),
            title: task_final_info.title.clone(),
            description: task_final_info.description.clone(),
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info.due_date.map(|d| DateTimeBridge::from_offset_date_time(d)),
            project_id: task_final_info.project_id,
            assignee_id: task_final_info.assignee_id,
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
            
        }
    }
            

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let task = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            RETURNING id, created_at, updated_at, title, description, owner_id, status, priority, due_date, project_id, assignee_id, labels
            "#,
            id,
        )
        .fetch_one(&plexo_engine.pool).await.unwrap();

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
            assignee_id: task.assignee_id,
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        let member = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, name, email, github_id, google_id, photo_url, role
            FROM members
            WHERE id = $1
            "#,
            id.clone(),
        ).fetch_one(&plexo_engine.pool).await.unwrap();




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
        description: Option<String>,
        owner_id: Uuid,
        labels: Option<Vec<String>>,
        prefix: String,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        
        let project_create = sqlx::query!(
            r#"
            INSERT INTO projects (name, owner_id, prefix)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            name,
            owner_id,
            prefix,
        ).fetch_one(&plexo_engine.pool).await.unwrap();

        // if description.is_some() {
        //     let _project_update_description = sqlx::query!(
        //         r#"
        //         UPDATE projects
        //         SET description = $1
        //         WHERE id = $2
        //         "#,
        //         description.unwrap(),
        //         project_create.id.clone(),
        //     ).execute(&plexo_engine.pool).await.unwrap();
        
        // }

        // if prefix.is_some() {
        //     let _project_update_prefix = sqlx::query!(
        //         r#"
        //         UPDATE projects
        //         SET prefix = $1
        //         WHERE id = $2
        //         "#,
        //         prefix.unwrap(),
        //         project_create.id.clone(),
        //     ).execute(&plexo_engine.pool).await.unwrap();
        
        // }

        // if labels.is_some() {
        //     for label in labels.unwrap() {
        //         let _project_create_label = sqlx::query!(
        //             r#"
        //             INSERT INTO labels (name, project_id)
        //             VALUES ($1, $2)
        //             "#,
        //             label,
        //             project_create.id.clone(),
        //         ).execute(&plexo_engine.pool).await.unwrap();
        //     }
        // }

        let project = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, name, owner_id, prefix
            FROM projects
            WHERE id = $1
            "#,
            project_create.id.clone(),
        ).fetch_one(&plexo_engine.pool).await.unwrap();
        
        Project {
            id: project.id,
            created_at: DateTimeBridge::from_offset_date_time(project.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
            name: project.name.clone(),
            description: None,
            owner_id: project.owner_id.unwrap_or(Uuid::nil()),
            prefix: project.prefix.clone(),
           
        }
        
        
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        labels: Option<Vec<String>>,
        prefix: Option<String>,
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
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }


        // if description.is_some() {
        //     let _project_update_description = sqlx::query!(
        //         r#"
        //         UPDATE projects
        //         SET description = $1
        //         WHERE id = $2
        //         "#,
        //         description.unwrap(),
        //         id.clone(),
        //     ).execute(&plexo_engine.pool).await.unwrap();
        
        // }

        if prefix.is_some() {
            let _project_update_prefix = sqlx::query!(
                r#"
                UPDATE projects
                SET prefix = $1
                WHERE id = $2
                "#,
                prefix.unwrap(),
                id.clone(),
            ).execute(&plexo_engine.pool).await.unwrap();
        
        }

        // if labels.is_some() {
        //     let _project_update_labels = sqlx::query!(
        //         r#"
        //         UPDATE projects
        //         SET labels = $1
        //         WHERE id = $2
        //         "#,
        //         labels.unwrap(),
        //         id.clone(),
        //     ).execute(&plexo_engine.pool).await.unwrap();
        
        // }


        let project = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, name, owner_id, prefix
            FROM projects
            WHERE id = $1
            "#,
            id.clone(),
        ).fetch_one(&plexo_engine.pool).await.unwrap();

        Project {
            id: project.id,
            created_at: DateTimeBridge::from_offset_date_time(project.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
            name: project.name.clone(),
            description: None,
            owner_id: project.owner_id.unwrap_or(Uuid::nil()),
            prefix: project.prefix.clone(),
           
        }



    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        
        let project = sqlx::query!(
        r#"
        DELETE FROM projects
        WHERE id = $1
        RETURNING id, created_at, updated_at, name, owner_id, prefix;
        "#,
            id,
        ).fetch_one(&plexo_engine.pool).await.unwrap();

        Project {
            id: project.id,
            created_at: DateTimeBridge::from_offset_date_time(project.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(project.updated_at),
            name: project.name.clone(),
            description: None,
            owner_id: project.owner_id.unwrap_or(Uuid::nil()),
            prefix: project.prefix.clone(),
           
        }
        
    }

}      