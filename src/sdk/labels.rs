use std::{collections::HashMap, sync::Arc};

use crate::{auth::auth::PlexoAuthToken, sdk::utilities::DateTimeBridge, system::core::Engine};
use async_graphql::{
    dataloader::{DataLoader, Loader},
    ComplexObject, Context, SimpleObject,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::task::{Task, TaskPriority, TaskStatus};
use super::loaders::TaskLoader;



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
        println!("token: {}", auth_token);

        let loader = ctx.data::<DataLoader<TaskLoader>>().unwrap();
        
        let ids : Vec<Uuid>= sqlx::query!(
            r#"
            SELECT task_id FROM labels_by_tasks
            WHERE label_id = $1
            "#,
            &self.id
        )
        .fetch_all(&plexo_engine.pool)
        .await
        .unwrap().into_iter().map(|id| id.task_id).collect();

        let tasks_map = loader.load_many(ids.clone()).await.unwrap();

        let tasks: &Vec<Task> = &ids
            .into_iter()
            .map(|id|  {
                tasks_map.get(&id).unwrap().clone()
        } )
        .collect();

        tasks.clone()
    }
}
