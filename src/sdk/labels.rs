use std::{collections::HashMap, sync::Arc};

use crate::{auth::auth::PlexoAuthToken, sdk::utilities::DateTimeBridge, system::core::Engine};
use async_graphql::{
    dataloader::{DataLoader, Loader},
    ComplexObject, Context, SimpleObject,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::task::{Task, TaskPriority, TaskStatus};

#[derive(Clone)]
pub struct TaskLoader(Engine);

impl TaskLoader {
    pub fn new(e: Engine) -> Self {
        Self(e)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TaskLoader {
    type Value = Task;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &'_ [Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let tasks: Vec<Task> = sqlx::query!(
            r#"
            SELECT * FROM tasks WHERE id  = ANY($1)
            "#,
            &keys
        )
        .fetch_all(&self.0.pool)
        .await
        .unwrap()
        .iter()
        .map(|task| Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title.clone(),
            description: task.description.clone(),
            owner_id: task.owner_id,
            status: TaskStatus::from_optional_str(&task.status),
            priority: TaskPriority::from_optional_str(&task.priority),
            due_date: task
                .due_date
                .map(|date| DateTimeBridge::from_offset_date_time(date)),
            project_id: task.project_id,
            lead_id: task.lead_id,
            count: task.count,
        })
        .collect();

        println!("{:?}", tasks);

        // let tasks_by_label: HashMap<Uuid, Vec<Task>> = HashMap::new();

        // Ok(tasks_by_label)

        Ok(HashMap::new())
    }
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Label {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[ComplexObject]
impl Label {
    pub async fn tasks(&self, ctx: &Context<'_>) -> Vec<Task> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let loader = ctx.data::<DataLoader<TaskLoader>>().unwrap();

        println!("loading tasks for label: {:?}", self.id);

        let tasks = loader.load_one(self.id).await.unwrap();

        // tasks.unwrap()

        vec![]

        // let tasks = sqlx::query!(
        //     r#"
        //     SELECT * FROM tasks
        //     WHERE id IN (
        //         SELECT task_id FROM labels_by_tasks
        //         WHERE label_id = $1
        //     )
        //     "#,
        //     &self.id
        // )
        // .fetch_all(&plexo_engine.pool)
        // .await
        // .unwrap();

        // tasks
        //     .into_iter()
        //     .map(|task| Task {
        //         id: task.id,
        //         created_at: DateTimeBridge::from_offset_date_time(task.created_at),
        //         updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
        //         title: task.title.clone(),
        //         description: task.description.clone(),
        //         owner_id: task.owner_id,
        //         status: TaskStatus::from_optional_str(&task.status),
        //         priority: TaskPriority::from_optional_str(&task.priority),
        //         due_date: task
        //             .due_date
        //             .map(|date| DateTimeBridge::from_offset_date_time(date)),
        //         project_id: task.project_id,
        //         lead_id: task.lead_id,
        //         count: task.count,
        //     })
        //     .collect()
    }
}
