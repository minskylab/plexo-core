use std::sync::mpsc::RecvError;
use std::{time::Duration, pin::Pin, sync::mpsc};
use async_graphql::{Enum, SimpleObject};
use tokio_stream::wrappers::IntervalStream;
use async_graphql::{futures_util::StreamExt, Context, Subscription, async_stream::stream, Object, FieldResult};
use chrono::Utc;
use tokio::{time::Instant, sync::futures};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use tokio_stream::Stream;
use uuid::{uuid, Uuid};

use crate::{
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
    },
    system::{
        core::Engine,
        subscriptions::SubscriptionManager,
    },
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum DataDiffEventKind {
    Created,
    Updated,
    Deleted,
}

#[derive(SimpleObject, Clone)]
pub struct DataDiffEvent {
    pub kind: DataDiffEventKind,
    pub data: String,
}

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
    
    async fn subscribe(&self, ctx: &Context<'_>) -> FieldResult<Pin<Box<dyn Stream<Item = String> + Send>>>{
        let (sender, receiver) = mpsc::channel();
        let subscription_manager = &ctx.data::<Engine>().unwrap().subscription_manager;
    
        let suscription_added = subscription_manager.add_subscription("subscription_id".to_string(), sender).await?;
        if (suscription_added == "subscription_id".to_string()) {
            println!("Subscription added");
        }
        let interval_stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));
        let mapped_stream = interval_stream.map(move |_| 
            if (receiver.try_recv().is_err()) {
                "No hay eventos recibidos".to_string()
            } else {
                "Task creado".to_string()
            }
        );
    
        Ok(Box::pin(mapped_stream))    
    }

    // async fn diffs(&self, ctx: &Context<'_>) -> FieldResult<Pin<Box<dyn Stream<Item = DataDiffEvent> + Send>>> {
    //     let stream_name = "hola".to_string();

    //     let interval_stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));
    //     let mapped_stream = interval_stream.map(move |_| DataDiffEvent {
    //         kind: DataDiffEventKind::Created,
    //         data: Uuid::new_v4().to_string(),
    //     });

    //     Ok(Box::pin(mapped_stream))
    // }

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
                prefix: None,
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
            })
    }
}
