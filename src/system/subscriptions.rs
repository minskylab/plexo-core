use crate::sdk::project::Project;
use crate::sdk::task::Task;
use crate::sdk::team::Team;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub enum DataContainer {
    TaskContainer(Task),
    ProjectContainer(Project),
    TeamContainer(Team),
}
pub struct Subscription {
    // id: String,
    sender: Sender<DataContainer>,
}

impl Subscription {
    fn new(_id: String, sender: Sender<DataContainer>) -> Self {
        Subscription { sender }
    }
}

type MyResult<T> = std::result::Result<T, String>;

#[derive(Clone)]
pub struct SubscriptionManager {
    pub subscriptions: Arc<Mutex<HashMap<String, Subscription>>>,
    pub id_task: String,
    pub id_project: String,
    pub id_team: String,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            id_task: Uuid::new_v4().to_string(),
            id_project: Uuid::new_v4().to_string(),
            id_team: Uuid::new_v4().to_string(),
        }
    }

    pub async fn add_subscription(
        &self,
        sender: Sender<DataContainer>,
        option: i32,
    ) -> MyResult<String> {
        let mut subscriptions = self.subscriptions.lock().await;

        if option == 1 {
            if subscriptions.contains_key(&self.id_task) {
                return Err(Box::<dyn Error>::from(format!(
                    "Subscription with id '{}' already exists",
                    self.id_task
                ))
                .to_string());
            }

            subscriptions.insert(
                self.id_task.clone(),
                Subscription::new(self.id_task.clone(), sender),
            );
            Ok(self.id_task.clone())
        } else if option == 2 {
            if subscriptions.contains_key(&self.id_project) {
                return Err(Box::<dyn Error>::from(format!(
                    "Subscription with id '{}' already exists",
                    self.id_project
                ))
                .to_string());
            }

            subscriptions.insert(
                self.id_project.clone(),
                Subscription::new(self.id_project.clone(), sender),
            );
            Ok(self.id_project.clone())
        } else {
            if subscriptions.contains_key(&self.id_team) {
                return Err(Box::<dyn Error>::from(format!(
                    "Subscription with id '{}' already exists",
                    self.id_team
                ))
                .to_string());
            }

            subscriptions.insert(
                self.id_team.clone(),
                Subscription::new(self.id_team.clone(), sender),
            );
            Ok(self.id_team.clone())
        }
    }

    async fn _remove_subscription(&self, id: String) -> MyResult<bool> {
        let mut subscriptions = self.subscriptions.lock().await;

        if !subscriptions.contains_key(&id) {
            return Ok(false);
        }

        subscriptions.remove(&id);
        Ok(true)
    }

    pub async fn send_task_event(&self, event: Task) -> MyResult<Task> {
        let mut subscriptions = self.subscriptions.lock().await;

        if let Some(subscription) = subscriptions.get_mut(&self.id_task) {
            subscription
                .sender
                .clone()
                .try_send(DataContainer::TaskContainer(event.clone()))
                .expect("Fallo al enviar el evento");
        }
        Ok(event)
    }

    pub async fn send_project_event(&self, event: Project) -> MyResult<Project> {
        let mut subscriptions = self.subscriptions.lock().await;

        if let Some(subscription) = subscriptions.get_mut(&self.id_project) {
            subscription
                .sender
                .clone()
                .try_send(DataContainer::ProjectContainer(event.clone()))
                .expect("Fallo al enviar el evento");
        }

        Ok(event)
    }

    pub async fn send_team_event(&self, event: Team) -> MyResult<Team> {
        let mut subscriptions = self.subscriptions.lock().await;

        if let Some(subscription) = subscriptions.get_mut(&self.id_team) {
            subscription
                .sender
                .clone()
                .try_send(DataContainer::TeamContainer(event.clone()))
                .expect("Fallo al enviar el evento");
        }

        Ok(event)
    }
}
