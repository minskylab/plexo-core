use async_graphql::{Enum, SimpleObject};
use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Instant};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::{Stream, StreamExt};

use uuid::Uuid;

// use tokio_stream::StreamExt;
// use uuid::Uuid;

pub struct Subscription {
    id: String,
    sender: Sender<String>,
}

impl Subscription {
    fn new(id: String, sender: Sender<String>) -> Self {
        Subscription {
            id: id,
            sender: sender,
        }
    }
}

type MyResult<T> = std::result::Result<T, String>;


#[derive(Clone)]
pub struct SubscriptionManager {
    pub subscriptions: Arc<Mutex<HashMap<String, Subscription>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_subscription(&self, id: String, sender: Sender<String>) -> MyResult<String> {
        let mut subscriptions = self.subscriptions.lock().await;
    
        if subscriptions.contains_key(&id) {
            return Err(Box::<dyn Error>::from(format!(
                "Subscription with id '{}' already exists",
                id
            )).to_string());
        }
    
        subscriptions.insert(id.clone(), Subscription::new(id.clone(), sender));
        Ok(id.clone())
    }

    async fn remove_subscription(&self, id: String) -> MyResult<bool>{
        let mut subscriptions = self.subscriptions.lock().await;

        if !subscriptions.contains_key(&id) {
            return Ok(false);
        }

        subscriptions.remove(&id);
        Ok(true)
    }

    pub async fn send_event(&self, id: String, event: String) -> MyResult<()> {
        let subscriptions = self.subscriptions.lock().await;

        if let Some(subscription) = subscriptions.get(&id) {
            subscription.sender.clone().send(event).unwrap();
        }

        Ok(())
    }
}

