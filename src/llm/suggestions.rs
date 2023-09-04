use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use serde_json::Result;
use sqlx::{query, Pool, Postgres};
use uuid::Uuid;

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
pub struct TaskSuggestionInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct TaskSuggestionResult {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: DateTime<Utc>,
}

#[derive(SimpleObject, Clone, Deserialize)]
pub struct SuggestionContext {
    project_id: Option<Uuid>,
    team_id: Option<Uuid>,
}

impl AutoSuggestionsEngine {
    pub fn new(pool: Box<Pool<Postgres>>) -> Self {
        let llm_engine = LLMEngine::new();
        Self { llm_engine, pool }
    }

    fn calculate_task_fingerprint(task: Task) -> String {
        serde_json::to_string(&task).unwrap()
    }

    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String {
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
            LIMIT 10
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

    pub async fn get_suggestions(
        &self,
        proto_task: TaskSuggestionInput,
        _context: Option<SuggestionContext>,
    ) -> Result<TaskSuggestionResult> {
        let tasks_fingerprints = self.acquire_tasks_fingerprints().await;

        let system_message = "The user pass to you a list of tasks and you should predict the following based on the input of the user.
        Please return only a valid json with the following struct {
                title: String,
                description: String,
                status: TaskStatus,
                priority: TaskPriority,
                due_date: DateTime<Utc>
        }".to_string();

        let user_message = format!(
            "
            Current Time:
            {}

            Current Tasks Context: 
            {}
            
            With the above context, complete the following task, only fill the <suggest> fields:
            {}",
            Local::now(),
            tasks_fingerprints.join("\n\n"),
            Self::calculate_task_suggestion_fingerprint(proto_task),
        );

        let result = self
            .llm_engine
            .chat_completion(system_message, user_message)
            .await;

        let suggestion_result: TaskSuggestionResult = serde_json::from_str(&result)?;

        Ok(suggestion_result)
    }

    pub async fn subdivide_task(
        &self,
        task_id: Uuid,
        subtasks: u32,
    ) -> Result<Vec<TaskSuggestionResult>> {
        let task = sqlx::query!(
            r#"
            SELECT * FROM tasks
            WHERE id = $1
            "#,
            task_id
        )
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        let task = Task {
            id: task.id,
            created_at: DateTimeBridge::from_offset_date_time(task.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(task.updated_at),
            title: task.title.clone(),
            description: task.description.clone(),
            status: TaskStatus::from_optional_str(&task.status),
            priority: TaskPriority::from_optional_str(&task.priority),
            due_date: task.due_date.map(DateTimeBridge::from_offset_date_time),
            project_id: task.project_id,
            lead_id: task.lead_id,
            owner_id: task.owner_id,
            count: task.count,
            parent_id: task.parent_id,
        };

        let system_message =
            "The user pass to you one task and you should predict a list of subtasks.
        Please return only a valid json with the following struct [{
                title: String,
                description: String,
                status: TaskStatus,
                priority: TaskPriority,
                due_date: DateTime<Utc>
        }]
        For TaskStatus and TaskPriority, please use the following values:
        TaskStatus: None, Backlog, ToDo, InProgress, Done, Canceled
        TaskPriority: None, Low, Medium, High, Urgent
        "
            .to_string();

        let user_message = format!(
            "
            Current Time:
            {}

            Parent Task: 
            {}
            
            With the above context, generate {} subtasks.",
            Local::now(),
            Self::calculate_task_fingerprint(task),
            subtasks,
        );

        let result = self
            .llm_engine
            .chat_completion(system_message, user_message)
            .await;

        let subtasks: Vec<TaskSuggestionResult> = serde_json::from_str(&result)?;

        Ok(subtasks)
    }

    // pub async fn get_
}
