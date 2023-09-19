use std::{collections::HashMap, sync::Arc};

use crate::system::core::Engine;
use async_graphql::dataloader::Loader;

use uuid::Uuid;

use super::{
    labels::Label,
    member::{Member, MemberRole},
    project::Project,
    task::{Task, TaskPriority, TaskStatus},
    team::{Team, TeamVisibility},
    utilities::DateTimeBridge,
};

pub struct TaskLoader(Engine);
pub struct ProjectLoader(Engine);
pub struct MemberLoader(Engine);
pub struct LabelLoader(Engine);
pub struct TeamLoader(Engine);

impl TaskLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

impl ProjectLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

impl MemberLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

impl LabelLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

impl TeamLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TaskLoader {
    type Value = Task;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tasks = sqlx::query!(
            r#"
            SELECT * FROM tasks WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let tasks_map: HashMap<Uuid, Task> = tasks
            .iter()
            .map(|task| {
                (
                    task.id,
                    Task {
                        id: task.id,
                        created_at: DateTimeBridge::from_offset_date_time(task.created_at),
                        updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
                        title: task.title.clone(),
                        description: task.description.clone(),
                        owner_id: task.owner_id,
                        status: TaskStatus::from_optional_str(&task.status),
                        priority: TaskPriority::from_optional_str(&task.priority),
                        due_date: task.due_date.map(DateTimeBridge::from_offset_date_time),
                        project_id: task.project_id,
                        lead_id: task.lead_id,
                        count: task.count,
                        parent_id: task.parent_id,
                    },
                )
            })
            .collect();

        // println!("{:?}", tasks_map);
        Ok(tasks_map)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let projects = sqlx::query!(
            r#"
            SELECT * FROM projects WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let projects_map: HashMap<Uuid, Project> = projects
            .iter()
            .map(|project| {
                (
                    project.id,
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
                            .map(DateTimeBridge::from_offset_date_time),
                        due_date: project.due_date.map(DateTimeBridge::from_offset_date_time),
                    },
                )
            })
            .collect();

        //println!("{:?}", projects);
        Ok(projects_map)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for MemberLoader {
    type Value = Member;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let members = sqlx::query!(
            r#"
            SELECT * FROM members WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let members_map: HashMap<Uuid, Member> = members
            .iter()
            .map(|member| {
                (
                    member.id,
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
                        password_hash: None,
                    },
                )
            })
            .collect();

        //println!("{:?}", members);
        Ok(members_map)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for LabelLoader {
    type Value = Label;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let labels = sqlx::query!(
            r#"
            SELECT * FROM labels WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let labels_map: HashMap<Uuid, Label> = labels
            .iter()
            .map(|r| {
                (
                    r.id,
                    Label {
                        id: r.id,
                        created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                        updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                        name: r.name.clone(),
                        color: r.color.clone(),
                        description: r.description.clone(),
                    },
                )
            })
            .collect();

        // println!("{:?}", labels_map);
        Ok(labels_map)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TeamLoader {
    type Value = Team;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let teams = sqlx::query!(
            r#"
            SELECT * FROM teams WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&*self.0.pool)
        .await
        .unwrap();

        //iterate to get the hashmap
        let teams_map: HashMap<Uuid, Team> = teams
            .iter()
            .map(|team| {
                (
                    team.id,
                    Team {
                        id: team.id,
                        created_at: DateTimeBridge::from_offset_date_time(team.created_at),
                        updated_at: DateTimeBridge::from_offset_date_time(team.updated_at),
                        name: team.name.clone(),
                        owner_id: team.owner_id,
                        visibility: TeamVisibility::from_optional_str(&team.visibility),
                        prefix: team.prefix.clone(),
                    },
                )
            })
            .collect();

        //println!("ga:{:?}", teams_map);

        Ok(teams_map)
    }
}
