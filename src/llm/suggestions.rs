use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{query, Pool, Postgres};

use crate::sdk::{
    task::{Task, TaskPriority, TaskStatus},
    utilities::DateTimeBridge,
};

use super::openai::LLMEngine;

#[derive(Clone)]
pub struct AutoSuggestionsEngine {
    llm_engine: LLMEngine,
    pool: Box<Pool<Postgres>>,
}

#[derive(InputObject, Clone)]
pub struct TaskSuggestion {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Clone)]
pub struct TaskSuggestionResult {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: DateTime<Utc>,
}

impl AutoSuggestionsEngine {
    pub fn new(pool: Box<Pool<Postgres>>) -> Self {
        let llm_engine = LLMEngine::new();
        Self { llm_engine, pool }
    }

    fn calculate_task_fingerprint(task: Task) -> String {
        format!(
            "Task Title: {}
            Task Description: {}
            Task Status: {}
            Task Priority: {}
            Task Created At: {}
            Task Due Date: {}",
            task.title,
            task.description.unwrap_or("<No description>".to_string()),
            task.status.to_str(),
            task.priority.to_str(),
            task.created_at,
            task.due_date
                .map(|d| d.to_rfc3339())
                .unwrap_or("<No due date>".to_string()),
        )
    }

    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestion) -> String {
        format!(
            "Task Title: {}
            Task Description: {}
            Task Status: {}
            Task Priority: {}
            Task Due Date: {}",
            task_suggestion.title.unwrap_or("<suggest>".to_string()),
            task_suggestion
                .description
                .unwrap_or("<suggest>".to_string()),
            task_suggestion
                .status
                .map(|s| s.to_str())
                .unwrap_or("<suggest>"),
            task_suggestion
                .priority
                .map(|p| p.to_str())
                .unwrap_or("<suggest>"),
            task_suggestion
                .due_date
                .map(|d| d.to_rfc3339())
                .unwrap_or("<suggest>".to_string()),
        )
    }

    async fn acquire_tasks_fingerprints(&self) -> Vec<String> {
        let tasks = query!(
            r#"
            SELECT *
            FROM tasks
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        tasks
            .iter()
            .map(|r| Task {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                title: r.title.clone(),
                description: r.description.clone(),
                status: TaskStatus::from_optional_str(&r.status),
                priority: TaskPriority::from_optional_str(&r.priority),
                due_date: r.due_date.map(DateTimeBridge::from_offset_date_time),
                project_id: r.project_id,
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
                parent_id: r.parent_id,
            })
            .map(Self::calculate_task_fingerprint)
            .collect::<Vec<String>>()
    }

    pub async fn get_suggestions(&self, proto_task: TaskSuggestion) -> String {
        let tasks_fingerprints = self.acquire_tasks_fingerprints().await;

        let user_message = format!(
            "Current Tasks Context: 
{}
With the above context, Complete the following task, only fill the <suggest> fields:
{}
",
            tasks_fingerprints.join("\n\n"),
            Self::calculate_task_suggestion_fingerprint(proto_task),
        );

        let system_message = "System: ".to_string();

        self.llm_engine
            .chat_completion(system_message, user_message)
            .await
    }
}
