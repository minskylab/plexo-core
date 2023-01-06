use std::time::Duration;

use async_graphql::{futures_util::StreamExt, Context, Subscription};
use chrono::Utc;
use tokio_stream::Stream;
use uuid::Uuid;

use crate::sdk::task::{Task, TaskPriority, TaskStatus};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }

    async fn tasks(&self, ctx: &Context<'_>) -> impl Stream<Item = Task> {
        let auth_token = ctx.data::<String>().unwrap();
        println!("token: {}", auth_token);

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(|_| Task {
                id: Uuid::new_v4(),
                title: "Task 1".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                description: None,

                status: TaskStatus::Backlog,
                priority: TaskPriority::High,

                owner_id: Uuid::new_v4(),

                labels: vec![],

                assignee_id: None,
                project_id: None,

                due_date: None,
            })
    }

    async fn task_by_id(&self, id: Uuid) -> impl Stream<Item = Task> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| Task {
                id,
                title: "Task 1".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                description: None,

                status: TaskStatus::Backlog,
                priority: TaskPriority::High,

                owner_id: Uuid::new_v4(),

                labels: vec![],

                assignee_id: None,
                project_id: None,

                due_date: None,
            })
    }
}
