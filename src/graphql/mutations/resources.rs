use async_graphql::{Context, InputObject, Object, Result};
use chrono::{DateTime, Utc};
use sqlx;
use uuid::Uuid;

use crate::{
    errors::definitions::PlexoAppError,
    graphql::auth::extract_context,
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

#[derive(InputObject)]
struct AssigneesOperation {
    _append: Option<Vec<Uuid>>,
    _delete: Option<Vec<Uuid>>,
}

#[derive(InputObject)]
struct CreateTaskInput {
    title: String,
    description: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    due_date: Option<DateTime<Utc>>,
    project_id: Option<Uuid>,
    lead_id: Option<Uuid>,
    labels: Option<Vec<Uuid>>,
    // assignees: Option<Vec<Uuid>>,
    parent_id: Option<Uuid>,
}

#[derive(Default)]
pub struct ResourcesMutation;

#[Object]
impl ResourcesMutation {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: Option<String>,
        status: Option<String>,
        priority: Option<String>,
        due_date: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        lead_id: Option<Uuid>,
        labels: Option<Vec<Uuid>>,
        assignees: Option<Vec<Uuid>>,
        parent_id: Option<Uuid>,
        subtasks: Option<Vec<CreateTaskInput>>,
    ) -> Result<Task> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        let task_final_info = sqlx::query!(r#"
            INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id, parent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING * 
            "#,
            title,
            description,
            member_id,
            status,
            priority,
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
            project_id,
            lead_id,
            parent_id,
        ).fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        if let Some(assignees) = assignees {
            let _delete_assignees = sqlx::query!(
                // TODO: Update this bad implementation
                r#"
                DELETE FROM tasks_by_assignees
                WHERE task_id = $1
                "#,
                task_final_info.id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for assignee in assignees {
                let _add_assignee = sqlx::query!(
                    r#"
                    INSERT INTO tasks_by_assignees (task_id, assignee_id)
                    VALUES ($1, $2)
                    "#,
                    task_final_info.id,
                    assignee,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(labels) = labels {
            let _delete_labels = sqlx::query!(
                r#"
                DELETE FROM labels_by_tasks
                WHERE task_id = $1
                "#,
                task_final_info.id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for label in labels {
                let _add_label = sqlx::query!(
                    r#"
                    INSERT INTO labels_by_tasks (task_id, label_id)
                    VALUES ($1, $2)
                    "#,
                    task_final_info.id,
                    label,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        if let Some(subtasks_to_create) = subtasks {
            for subtask in subtasks_to_create {
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id, parent_id)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    RETURNING * 
                    "#,
                    subtask.title,
                    subtask.description,
                    member_id,
                    subtask.status,
                    subtask.priority,
                    subtask.due_date.map(|d| DateTimeBridge::from_date_time(d)),
                    subtask.project_id,
                    subtask.lead_id,
                    task_final_info.id,
                ).fetch_one(&*plexo_engine.pool)
                .await
                .unwrap();
            }

            // TODO: Implement subscription signal for subtasks
        }

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
                .map(DateTimeBridge::from_offset_date_time),
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        let _ = subscription_manager.send_task_event(task.clone()).await;

        Ok(task)
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
        labels: Option<Vec<Uuid>>,
        assignees: Option<Vec<Uuid>>,
    ) -> Result<Task> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let task_final_info = sqlx::query!(
            r#"
            UPDATE tasks
            SET 
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                status = COALESCE($3, status),
                priority = COALESCE($4, priority),
                due_date = COALESCE($5, due_date),
                project_id = COALESCE($6, project_id),
                lead_id = COALESCE($7, lead_id)
            WHERE id = $8
            RETURNING * 
            "#,
            title,
            description,
            status,
            priority,
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
            project_id,
            lead_id,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        if let Some(assignees) = assignees {
            let _delete_assignees = sqlx::query!(
                r#"
                    DELETE FROM tasks_by_assignees
                    WHERE task_id = $1
                    "#,
                task_final_info.id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for assignee in assignees {
                let _add_assignee = sqlx::query!(
                    r#"
                        INSERT INTO tasks_by_assignees (task_id, assignee_id)
                        VALUES ($1, $2)
                        "#,
                    task_final_info.id,
                    assignee,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(labels) = labels {
            let _delete_labels = sqlx::query!(
                r#"
                DELETE FROM labels_by_tasks
                WHERE task_id = $1
                "#,
                task_final_info.id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for label in labels {
                let _add_label = sqlx::query!(
                    r#"
                    INSERT INTO labels_by_tasks (task_id, label_id)
                    VALUES ($1, $2)
                    "#,
                    task_final_info.id,
                    label,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

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
                .map(DateTimeBridge::from_offset_date_time),
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        subscription_manager
            .send_task_event(task.clone())
            .await
            .unwrap();

        Ok(task)
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let task_final_info = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_assignees = sqlx::query!(
            r#"
            DELETE FROM tasks_by_assignees
            WHERE task_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_labels = sqlx::query!(
            r#"
            DELETE FROM labels_by_tasks
            WHERE task_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

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
                .map(DateTimeBridge::from_offset_date_time),
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        subscription_manager
            .send_task_event(task.clone())
            .await
            .unwrap();

        Ok(task)
    }

    async fn update_member(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        email: Option<String>,
        name: Option<String>,
        role: Option<String>,
        // projects: Option<Vec<Uuid>>,
        // teams: Option<Vec<Uuid>>,
    ) -> Result<Member> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let member = sqlx::query!(
            r#"
            UPDATE members
            SET email = $1, name = $2, role = $3
            WHERE id = $4
            RETURNING id, created_at, updated_at, email, name, github_id, google_id, photo_url, role
            "#,
            email,
            name,
            role,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        // if let Some(projects) = projects {
        //     let _deleted_projects = sqlx::query!(
        //         r#"
        //             DELETE FROM members_by_projects
        //             WHERE member_id = $1
        //             "#,
        //         id,
        //     )
        //     .execute(&*plexo_engine.pool)
        //     .await
        //     .unwrap();

        //     for project in projects {
        //         let _inserted_projects = sqlx::query!(
        //             r#"
        //                 INSERT INTO members_by_projects (member_id, project_id)
        //                 VALUES ($1, $2)
        //                 "#,
        //             id,
        //             project,
        //         )
        //         .execute(&*plexo_engine.pool)
        //         .await
        //         .unwrap();
        //     }
        // }

        // if let Some(teams) = teams {
        //     let _deleted_teams = sqlx::query!(
        //         r#"
        //             DELETE FROM members_by_teams
        //             WHERE member_id = $1
        //             "#,
        //         id,
        //     )
        //     .execute(&*plexo_engine.pool)
        //     .await
        //     .unwrap();

        //     for team in teams {
        //         let _inserted_teams = sqlx::query!(
        //             r#"
        //                 INSERT INTO members_by_teams (member_id, team_id)
        //                 VALUES ($1, $2)
        //                 "#,
        //             id,
        //             team,
        //         )
        //         .execute(&*plexo_engine.pool)
        //         .await
        //         .unwrap();
        //     }
        // }

        Ok(Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name.clone(),
            email: member.email.clone(),
            github_id: member.github_id,
            google_id: member.google_id,
            photo_url: member.photo_url,
            role: MemberRole::from_optional_str(&member.role),
            password_hash: None,
        })
    }

    // async fn delete_member(&self, ctx: &Context<'_>, id: Uuid) -> Result<Member> {
    //     let (plexo_engine, _member_id) = extract_context(ctx)?;

    //     let member = sqlx::query!(
    //         r#"
    //         DELETE FROM members
    //         WHERE id = $1
    //         RETURNING id, created_at, updated_at, name, email, github_id, google_id, photo_url, role;
    //         "#,
    //         id,
    //     )
    //     .fetch_one(&*plexo_engine.pool).await.unwrap();

    //     let _deleted_projects = sqlx::query!(
    //         r#"
    //         DELETE FROM members_by_projects
    //         WHERE member_id = $1
    //         "#,
    //         id,
    //     )
    //     .execute(&*plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     let _deleted_teams = sqlx::query!(
    //         r#"
    //         DELETE FROM members_by_teams
    //         WHERE member_id = $1
    //         "#,
    //         id,
    //     )
    //     .execute(&*plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     let _deleted_tasks = sqlx::query!(
    //         r#"
    //         DELETE FROM tasks_by_assignees
    //         WHERE assignee_id = $1
    //         "#,
    //         id,
    //     )
    //     .execute(&*plexo_engine.pool)
    //     .await
    //     .unwrap();

    //     Ok(Member {
    //         id: member.id,
    //         created_at: DateTimeBridge::from_offset_date_time(member.created_at),
    //         updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
    //         name: member.name.clone(),
    //         email: member.email.clone(),
    //         github_id: member.github_id,
    //         google_id: member.google_id,
    //         photo_url: member.photo_url,
    //         role: MemberRole::from_optional_str(&member.role),
    //     })
    // }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
        prefix: Option<String>,
        description: Option<String>,
        lead_id: Option<Uuid>,
        start_date: Option<DateTime<Utc>>,
        due_date: Option<DateTime<Utc>>,
        members: Option<Vec<Uuid>>,
        teams: Option<Vec<Uuid>>,
    ) -> Result<Project> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        let project = sqlx::query!(
            r#"
            INSERT INTO projects (name, prefix, owner_id, description, lead_id, start_date, due_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created_at, updated_at, name, prefix, owner_id, description, lead_id, start_date, due_date
            "#,
            name,
            prefix,
            member_id,
            description,
            lead_id,
            start_date.map(|d| DateTimeBridge::from_date_time(d)),
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
        )
        .fetch_one(&*plexo_engine.pool).await.unwrap();

        if let Some(members) = members {
            for member in members {
                let _inserted_members = sqlx::query!(
                    r#"
                        INSERT INTO members_by_projects (member_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    project.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(teams) = teams {
            for team in teams {
                let _inserted_teams = sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team,
                    project.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let project = Project {
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
        };

        subscription_manager
            .send_project_event(project.clone())
            .await
            .unwrap();

        Ok(project)
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
        members: Option<Vec<Uuid>>,
        teams: Option<Vec<Uuid>>,
    ) -> Result<Project> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let project = sqlx::query!(
            r#"
            UPDATE projects
            SET 
                name = COALESCE($1, name),
                prefix = COALESCE($2, prefix),
                owner_id = COALESCE($3, owner_id),
                description = COALESCE($4, description),
                lead_id = COALESCE($5, lead_id),
                start_date = COALESCE($6, start_date),
                due_date = COALESCE($7,due_date) 
            WHERE id = $8
            RETURNING *
            "#,
            name,
            prefix,
            owner_id,
            description,
            lead_id,
            start_date.map(|d| DateTimeBridge::from_date_time(d)),
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        if let Some(members) = members {
            let _deleted_members = sqlx::query!(
                r#"
                    DELETE FROM members_by_projects
                    WHERE project_id = $1
                    "#,
                id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for member in members {
                let _inserted_members = sqlx::query!(
                    r#"
                        INSERT INTO members_by_projects (member_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    project.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(teams) = teams {
            let _deleted_teams = sqlx::query!(
                r#"
                    DELETE FROM teams_by_projects
                    WHERE project_id = $1
                    "#,
                id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for team in teams {
                let _inserted_teams = sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team,
                    project.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let project = Project {
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
        };

        subscription_manager
            .send_project_event(project.clone())
            .await
            .unwrap();

        Ok(project)
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let project = sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_members = sqlx::query!(
            r#"
            DELETE FROM members_by_projects
            WHERE project_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_teams = sqlx::query!(
            r#"
            DELETE FROM teams_by_projects
            WHERE project_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
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
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let project = Project {
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
        };

        subscription_manager
            .send_project_event(project.clone())
            .await
            .unwrap();

        Ok(project)
    }

    async fn create_team(
        &self,
        ctx: &Context<'_>,
        name: String,
        visibility: Option<String>,
        prefix: Option<String>,
        members: Option<Vec<Uuid>>,
        projects: Option<Vec<Uuid>>,
    ) -> Result<Team> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        let team = sqlx::query!(
            r#"
            INSERT INTO teams (name, owner_id, visibility, prefix)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            name,
            member_id,
            visibility,
            prefix,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        if let Some(members) = members {
            for member in members {
                let _inserted_members = sqlx::query!(
                    r#"
                        INSERT INTO members_by_teams (member_id, team_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    team.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(projects) = projects {
            for project in projects {
                let _inserted_projects = sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team.id,
                    project,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let team = Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        };

        subscription_manager
            .send_team_event(team.clone())
            .await
            .unwrap();

        Ok(team)
    }

    async fn update_team(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        owner_id: Option<Uuid>,
        visibility: Option<String>,
        prefix: Option<String>,
        members: Option<Vec<Uuid>>,
        projects: Option<Vec<Uuid>>,
    ) -> Result<Team> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let team = sqlx::query!(
            r#"
            UPDATE teams
            SET
                name = COALESCE($1, name),
                owner_id = COALESCE($2, owner_id),
                visibility = COALESCE($3, visibility),
                prefix = COALESCE($4, prefix)
            WHERE id = $5
            RETURNING *
            "#,
            name,
            owner_id,
            visibility,
            prefix,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        if let Some(members) = members {
            let _deleted_members = sqlx::query!(
                r#"
                    DELETE FROM members_by_teams
                    WHERE team_id = $1
                    "#,
                id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for member in members {
                let _inserted_members = sqlx::query!(
                    r#"
                        INSERT INTO members_by_teams (member_id, team_id)
                        VALUES ($1, $2)
                        "#,
                    member,
                    team.id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        if let Some(projects) = projects {
            let _deleted_projects = sqlx::query!(
                r#"
                    DELETE FROM teams_by_projects
                    WHERE team_id = $1
                    "#,
                id,
            )
            .execute(&*plexo_engine.pool)
            .await
            .unwrap();

            for project in projects {
                let _inserted_projects = sqlx::query!(
                    r#"
                        INSERT INTO teams_by_projects (team_id, project_id)
                        VALUES ($1, $2)
                        "#,
                    team.id,
                    project,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();
            }
        }

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let team = Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        };

        subscription_manager
            .send_team_event(team.clone())
            .await
            .unwrap();
        Ok(team)
    }

    async fn delete_team(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let team = sqlx::query!(
            r#"
            DELETE FROM teams
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _delete_team_members = sqlx::query!(
            r#"
            DELETE FROM members_by_teams
            WHERE team_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let _delete_team_projects = sqlx::query!(
            r#"
            DELETE FROM teams_by_projects
            WHERE team_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let subscription_manager: &crate::system::subscriptions::SubscriptionManager =
            &ctx.data::<Engine>().unwrap().subscription_manager;

        let team = Team {
            id: team.id,
            created_at: DateTimeBridge::from_offset_date_time(team.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
            name: team.name.clone(),
            owner_id: team.owner_id,
            visibility: TeamVisibility::from_optional_str(&team.visibility),
            prefix: team.prefix.clone(),
        };

        subscription_manager
            .send_team_event(team.clone())
            .await
            .unwrap();
        Ok(team)
    }

    async fn create_label(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Label> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let label = sqlx::query!(
            r#"
            INSERT INTO labels (name, description, color)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            name,
            description,
            color,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color,
        })
    }

    async fn update_label(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Label> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let label = sqlx::query!(
            r#"
            UPDATE labels
            SET name = $1, description = $2, color = $3
            WHERE id = $4
            RETURNING *
            "#,
            name,
            description,
            color,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color,
        })
    }

    async fn delete_label(&self, ctx: &Context<'_>, id: Uuid) -> Result<Label> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let label = sqlx::query!(
            r#"
            DELETE FROM labels
            WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_labels = sqlx::query!(
            r#"
            DELETE FROM labels_by_tasks
            WHERE label_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        Ok(Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color.clone(),
        })
    }

    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
        email: Option<String>,
        photo_url: Option<String>,
    ) -> Result<Member> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        if email.is_some()
            && plexo_engine
                .get_member_by_email(email.clone().unwrap())
                .await
                .is_some()
        {
            return Err(PlexoAppError::EmailAlreadyInUse.into());
        }

        let r = sqlx::query!(
            r#"
            UPDATE 
                members
            SET 
                name = COALESCE($1, name), 
                email = COALESCE($2, email), 
                photo_url = COALESCE($3, photo_url)
            WHERE 
                id = $4
            RETURNING *
            "#,
            name,
            email,
            photo_url,
            member_id,
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
            photo_url: r.photo_url.clone(),
            github_id: r.github_id,
            google_id: r.google_id,
            role: MemberRole::from_optional_str(&r.role),
            password_hash: None,
        })
    }

    async fn update_password(
        &self,
        ctx: &Context<'_>,
        current_password: String,
        new_password: String,
    ) -> Result<Member> {
        let (plexo_engine, member_id) = extract_context(ctx)?;

        let member = sqlx::query!(
            r#"
            SELECT * FROM members
            WHERE id = $1
            "#,
            member_id,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let password_hash = member.password_hash.unwrap_or("".to_string());

        if current_password.is_empty() ^ password_hash.is_empty() {
            return Err(PlexoAppError::InvalidPassword.into());
        }

        if !current_password.is_empty()
            && !plexo_engine
                .auth
                .validate_password(&current_password, &password_hash)
        {
            return Err(PlexoAppError::InvalidPassword.into());
        }

        let new_password_hash = plexo_engine.auth.hash_password(&new_password);

        let r = sqlx::query!(
            r#"
            UPDATE members
            SET password_hash = $1
            WHERE id = $2
            RETURNING *
            "#,
            new_password_hash,
            member_id,
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
            photo_url: r.photo_url.clone(),
            github_id: r.github_id,
            google_id: r.google_id,
            role: MemberRole::from_optional_str(&r.role),
            password_hash: None,
        })
    }
}
