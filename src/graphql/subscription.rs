use std::time::Duration;

use std::{pin::Pin};
use async_graphql::{futures_util::StreamExt, Context, Subscription, async_stream::stream, FieldResult};
use chrono::Utc;
use tokio_stream::Stream;
use uuid::Uuid;
use tokio::sync::mpsc::{channel, Sender, Receiver};

use crate::system::subscriptions::DataContainer;
use crate::{
    sdk::{
        project::Project,
        task::{Task, TaskPriority, TaskStatus},
        team::{Team, TeamVisibility},
    },
    system::{
        core::Engine,
    },
};


#[derive(Default)]
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn subscribe_task(&self, ctx: &Context<'_>) -> FieldResult<Pin<Box<dyn Stream<Item = Option<Task>> + Send>>>{
        let (sender, mut receiver) = channel(100);
        let subscription_manager = &ctx.data::<Engine>().unwrap().subscription_manager;
        let new_uuid = Uuid::new_v4().to_string();

        let suscription_added = subscription_manager.add_subscription(sender,1).await?;
        if (suscription_added == new_uuid.clone()) {
            println!("Subscription_Task added");
        }

        let mapped_stream = stream! {
            let mut last_task: Option<Task>= None;
            loop {
                match receiver.recv().await {
                    Some(DataContainer::TaskContainer(task)) => {
                        println!("{}", task.title);
                        last_task = Some(task);
                        yield last_task.clone();
                    },
                    Some(DataContainer::ProjectContainer(task)) => {
                        yield None;

                    },
                    Some(DataContainer::TeamContainer(task)) => {
                        yield None;
                    },
                    None => {
                        println!("None");
                        yield None;
                    },
                }
            }
        };

    
        Ok(Box::pin(mapped_stream))    
    }

    async fn subscribe_project(&self, ctx: &Context<'_>) -> FieldResult<Pin<Box<dyn Stream<Item = Option<Project>> + Send>>>{
        let (sender, mut receiver) = channel(100);
        let subscription_manager = &ctx.data::<Engine>().unwrap().subscription_manager;
        let new_uuid = Uuid::new_v4().to_string();

        let suscription_added = subscription_manager.add_subscription(sender,2).await?;
        if (suscription_added == new_uuid.clone()) {
            println!("Subscription_Project added");
        }

        let mapped_stream = stream! {
            let mut last_task: Option<Project>= None;
            loop {
                match receiver.recv().await {
                    Some(DataContainer::TaskContainer(task)) => {
                        yield None;

                    },
                    Some(DataContainer::ProjectContainer(task)) => {
                        println!("{}", task.id);
                        last_task = Some(task);
                        yield last_task.clone();
                    },
                    Some(DataContainer::TeamContainer(task)) => {
                        yield None;
                    },
                    None => {
                        println!("None");
                        yield None;
                    },
                }
            }
        };

    
        Ok(Box::pin(mapped_stream))    
    }

    async fn subscribe_team(&self, ctx: &Context<'_>) -> FieldResult<Pin<Box<dyn Stream<Item = Option<Team>> + Send>>>{
        let (sender, mut receiver) = channel(100);
        let subscription_manager = &ctx.data::<Engine>().unwrap().subscription_manager;
        let new_uuid = Uuid::new_v4().to_string();

        let suscription_added = subscription_manager.add_subscription(sender,3).await?;
        if (suscription_added == new_uuid.clone()) {
            println!("Subscription_Team added");
        }

        let mapped_stream = stream! {
            let mut last_task: Option<Team>= None;
            loop {
                match receiver.recv().await {
                    Some(DataContainer::TaskContainer(task)) => {
                        yield None;

                    },
                    Some(DataContainer::ProjectContainer(task)) => {
                        yield None;

                    },
                    Some(DataContainer::TeamContainer(task)) => {
                        println!("{}", task.id);
                        last_task = Some(task);
                        yield last_task.clone();                    },
                    None => {
                        println!("None");
                        yield None;
                    },
                }
            }
        };

    
        Ok(Box::pin(mapped_stream))    
    }

    async fn tasks(&self, ctx: &Context<'_>) -> impl Stream<Item = Task> {
        let _auth_token = ctx.data::<String>().unwrap();

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

                // labels: vec![],
                lead_id: None,
                project_id: None,

                due_date: None,
                count: 0,
                parent_id: None,
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

                // labels: vec![],
                lead_id: None,
                project_id: None,

                due_date: None,
                count: 0,
                parent_id: None,
            })
    }

    async fn projects(&self, ctx: &Context<'_>) -> impl Stream<Item = Project> {
        let _auth_token = ctx.data::<String>().unwrap();

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(|_| Project {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                name: "Project X".to_string(),
                description: None,
                owner_id: Uuid::new_v4(),
                prefix: None,
                lead_id: None,
                start_date: None,
                due_date: None,
            })
    }

    async fn teams(&self, ctx: &Context<'_>) -> impl Stream<Item = Team> {
        let _auth_token = ctx.data::<String>().unwrap();

        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(|_| Team {
                id: Uuid::new_v4(),
                name: "Team X".to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                owner_id: Uuid::new_v4(),
                visibility: TeamVisibility::Public,
                prefix: None,
            })
    }
}
