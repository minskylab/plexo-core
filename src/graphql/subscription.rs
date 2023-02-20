use std::time::Duration;

use async_graphql::{futures_util::StreamExt, Context, Subscription};
use chrono::Utc;
use tokio_stream::Stream;
use uuid::{uuid, Uuid};

use crate::{
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
    },
    system::subscriptions::{DataDiffEvent, DataDiffEventKind},
};

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

    async fn example(&self, ctx: &Context<'_>) -> impl Stream<Item = DataDiffEvent> {
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| DataDiffEvent {
                kind: DataDiffEventKind::Created,
                data: Uuid::new_v4().to_string(),
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

                lead_id: None,
                project_id: None,

                due_date: None,
                count: 0,
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

                lead_id: None,
                project_id: None,

                due_date: None,
                count: 0,
            })
    }

    async fn projects(&self, ctx: &Context<'_>) -> impl Stream<Item = Project> {
        let auth_token = ctx.data::<String>().unwrap();
        println!("token: {}", auth_token);

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(|_| Project {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                name: "Project X".to_string(),
                description: None,
                owner_id: Uuid::new_v4(),
                prefix: "PX".to_string(),
                lead_id: None,
                start_date: None,
                due_date: None,
            })
    }

    async fn teams(&self, ctx: &Context<'_>) -> impl Stream<Item = Team> {
        let auth_token = ctx.data::<String>().unwrap();
        println!("token: {}", auth_token);

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(|_| Team {
                id: Uuid::new_v4(),
                name: "Team X".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                owner_id: Uuid::new_v4(),
                visibility: TeamVisibility::Public,
                prefix: "TX".to_string(),
            })
    }
}
