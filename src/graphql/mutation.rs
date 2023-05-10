use async_graphql::{Context, InputObject, Object};
use chrono::{DateTime, Utc};
use sqlx;
use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
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
        labels: Option<Vec<Uuid>>,
        assignees: Option<Vec<Uuid>>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let task_final_info = sqlx::query!(
            r#"
            INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING * 
            "#,
            title,
            description,
            owner_id,
            status,
            priority,
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
            project_id,
            lead_id
        ).fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        match assignees {
            Some(assignees) => {
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
            None => (),
        };

        match labels {
            Some(labels) => {
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
            None => (),
        };

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
            owner_id: task_final_info.owner_id,
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
        labels: Option<Vec<Uuid>>,
        assignees: Option<Vec<Uuid>>,
    ) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let task_final_info = sqlx::query!(
            r#"
            UPDATE tasks
            SET title = $1, description = $2, status = $3, priority = $4, due_date = $5, project_id = $6, lead_id = $7
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
        ).fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _a = match assignees {
            Some(assignees) => {
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
            None => (),
        };

        let _l = match labels {
            Some(labels) => {
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
            None => (),
        };

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
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
        };

        task
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Task {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let task_final_info = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            RETURNING id, created_at, updated_at, title, description, owner_id, status, priority, due_date, project_id, lead_id, count
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool).await.unwrap();

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
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
        };

        task
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
        role: Option<String>,
        projects: Option<Vec<Uuid>>,
        teams: Option<Vec<Uuid>>,
    ) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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

        let _a = match projects {
            Some(projects) => {
                let _deleted_projects = sqlx::query!(
                    r#"
                    DELETE FROM members_by_projects
                    WHERE member_id = $1
                    "#,
                    id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();

                for project in projects {
                    let _inserted_projects = sqlx::query!(
                        r#"
                        INSERT INTO members_by_projects (member_id, project_id)
                        VALUES ($1, $2)
                        "#,
                        id,
                        project,
                    )
                    .execute(&*plexo_engine.pool)
                    .await
                    .unwrap();
                }
            }
            None => (),
        };

        let _b = match teams {
            Some(teams) => {
                let _deleted_teams = sqlx::query!(
                    r#"
                    DELETE FROM members_by_teams
                    WHERE member_id = $1
                    "#,
                    id,
                )
                .execute(&*plexo_engine.pool)
                .await
                .unwrap();

                for team in teams {
                    let _inserted_teams = sqlx::query!(
                        r#"
                        INSERT INTO members_by_teams (member_id, team_id)
                        VALUES ($1, $2)
                        "#,
                        id,
                        team,
                    )
                    .execute(&*plexo_engine.pool)
                    .await
                    .unwrap();
                }
            }
            None => (),
        };

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
        println!("token: {}", auth_token);

        let member = sqlx::query!(
            r#"
            DELETE FROM members
            WHERE id = $1
            RETURNING id, created_at, updated_at, name, email, github_id, google_id, photo_url, role;
            "#,
            id,
        )
        .fetch_one(&*plexo_engine.pool).await.unwrap();

        let _deleted_projects = sqlx::query!(
            r#"
            DELETE FROM members_by_projects
            WHERE member_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_teams = sqlx::query!(
            r#"
            DELETE FROM members_by_teams
            WHERE member_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
        .await
        .unwrap();

        let _deleted_tasks = sqlx::query!(
            r#"
            DELETE FROM tasks_by_assignees
            WHERE assignee_id = $1
            "#,
            id,
        )
        .execute(&*plexo_engine.pool)
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
        members: Option<Vec<Uuid>>,
        teams: Option<Vec<Uuid>>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let project = sqlx::query!(
            r#"
            INSERT INTO projects (name, prefix, owner_id, description, lead_id, start_date, due_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created_at, updated_at, name, prefix, owner_id, description, lead_id, start_date, due_date
            "#,
            name,
            prefix,
            owner_id,
            description,
            lead_id,
            start_date.map(|d| DateTimeBridge::from_date_time(d)),
            due_date.map(|d| DateTimeBridge::from_date_time(d)),
        )
        .fetch_one(&*plexo_engine.pool).await.unwrap();

        let _a = match members {
            Some(members) => {
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
            None => (),
        };

        let _b = match teams {
            Some(teams) => {
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
            None => (),
        };

        Project {
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
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            due_date: project
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
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
        members: Option<Vec<Uuid>>,
        teams: Option<Vec<Uuid>>,
    ) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let project = sqlx::query!(
            r#"
            UPDATE projects
            SET name = $1, prefix = $2, owner_id = $3, description = $4, lead_id = $5, start_date = $6, due_date = $7
            WHERE id = $8
            RETURNING id, created_at, updated_at, name, prefix, owner_id, description, lead_id, start_date, due_date
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
        .fetch_one(&*plexo_engine.pool).await.unwrap();

        let _a = match members {
            Some(members) => {
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
            None => (),
        };

        let _b = match teams {
            Some(teams) => {
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
            None => (),
        };

        Project {
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
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            due_date: project
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
        }
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Project {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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

        Project {
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
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
            due_date: project
                .due_date
                .map(|d| DateTimeBridge::from_offset_date_time(d)),
        }
    }

    async fn create_team(
        &self,
        ctx: &Context<'_>,
        name: String,
        owner_id: Uuid,
        visibility: Option<String>,
        prefix: Option<String>,
        members: Option<Vec<Uuid>>,
        projects: Option<Vec<Uuid>>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let team = sqlx::query!(
            r#"
            INSERT INTO teams (name, owner_id, visibility, prefix)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            name,
            owner_id,
            visibility,
            prefix,
        )
        .fetch_one(&*plexo_engine.pool)
        .await
        .unwrap();

        let _a = match members {
            Some(members) => {
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
            None => (),
        };

        let _b = match projects {
            Some(projects) => {
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
            None => (),
        };

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
        members: Option<Vec<Uuid>>,
        projects: Option<Vec<Uuid>>,
    ) -> Team {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

        let team = sqlx::query!(
            r#"
            UPDATE teams
            SET name = $1, owner_id = $2, visibility = $3, prefix = $4
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

        let _a = match members {
            Some(members) => {
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
            None => (),
        };

        let _b = match projects {
            Some(projects) => {
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
            None => (),
        };

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
        println!("token: {}", auth_token);

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

    async fn create_label(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
        color: Option<String>,
    ) -> Label {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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

        Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color.clone(),
        }
    }

    async fn update_label(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    ) -> Label {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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

        Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color.clone(),
        }
    }

    async fn delete_label(&self, ctx: &Context<'_>, id: Uuid) -> Label {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        println!("token: {}", auth_token);

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

        Label {
            id: label.id,
            created_at: DateTimeBridge::from_offset_date_time(label.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(label.updated_at),
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color.clone(),
        }
    }
}
