use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use chrono::{DateTime, Utc};

use crate::{
    graphql::auth::extract_context,
    llm::suggestions::{TaskSuggestion, TaskSuggestionResult},
    sdk::task::{TaskPriority, TaskStatus},
};

#[derive(Default)]
pub struct AIFunctionsQuery;

#[Object]
impl AIFunctionsQuery {
    async fn suggest_new_task(
        &self,
        ctx: &Context<'_>,
        task: TaskSuggestion,
    ) -> Result<TaskSuggestionResult> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let raw_suggestion = plexo_engine
            .auto_suggestions_engine
            .get_suggestions(task)
            .await;

        let parts = raw_suggestion
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let title = parts[0].replace("Task Title:", "").trim().to_string();
        let description = parts[1].replace("Task Description:", "").trim().to_string();
        let status = parts[2].replace("Task Status:", "").trim().to_string();
        let priority = parts[3].replace("Task Priority:", "").trim().to_string();
        let due_date = parts[4].replace("Task Due Date:", "").trim().to_string();

        let status = TaskStatus::from_str(&status).unwrap_or_default();
        let priority = TaskPriority::from_str(&priority).unwrap_or_default();
        let due_date = DateTime::<Utc>::from_str(&due_date).unwrap_or(Utc::now());

        Ok(TaskSuggestionResult {
            title,
            description,
            status,
            priority,
            due_date,
        })
    }

    async fn subdivide_task(&self, ctx: &Context<'_>, task_id: String) -> Result<String> {
        let (_plexo_engine, _member_id) = extract_context(ctx)?;

        let task_id = task_id.parse::<uuid::Uuid>()?;

        // let task = plexo_engine.task_engine.get_task_by_id(task_id).await?;

        // let new_task = plexo_engine
        //     .task_engine
        //     .create_task(
        //         task.title,
        //         task.description,
        //         task.status,
        //         task.priority,
        //         task.due_date,
        //     )
        //     .await?;

        // plexo_engine
        //     .task_engine
        //     .add_subtask(task_id, new_task.id)
        //     .await?;

        Ok(task_id.to_string())
    }
}
