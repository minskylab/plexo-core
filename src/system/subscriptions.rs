
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
//use uuid::Uuid;
use crate::sdk::task::Task;

pub struct Subscription {
    id: String,
    sender: Sender<Task>,
}

impl Subscription {
    fn new(id: String, sender: Sender<Task>) -> Self {
        Subscription {
            id: id,
            sender: sender,
        }
    }
}

type MyResult<T> = std::result::Result<T, String>;
type MyResultTask<T> = std::result::Result<T, Task>;


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

    pub async fn add_subscription(&self, id: String, sender: Sender<Task>) -> MyResult<String> {
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

    pub async fn send_event(&self, id: String, event: Task) -> MyResult<Task> {
        let mut subscriptions = self.subscriptions.lock().await;

        if let Some(subscription) = subscriptions.get_mut(&id) {
            subscription.sender.clone().try_send(event.clone());
        }

        Ok(event)
    }
}

