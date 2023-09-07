use async_graphql::{Context, Object, Result};

use crate::{
    graphql::auth::extract_context,
    llm::suggestions::{TaskSuggestionInput, TaskSuggestionResult},
};

#[derive(Default)]
pub struct AIFunctionsQuery;

#[Object]
impl AIFunctionsQuery {
    async fn suggest_new_task(
        &self,
        ctx: &Context<'_>,
        task: TaskSuggestionInput,
    ) -> Result<TaskSuggestionResult> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let Ok(raw_suggestion) = plexo_engine
            .auto_suggestions_engine
            .get_suggestions(task, None)
            .await
        else {
            return Err("Failed to get suggestions".into());
        };

        Ok(raw_suggestion)
    }

    async fn subdivide_task(
        &self,
        ctx: &Context<'_>,
        task_id: String,
        #[graphql(default = 3)] subtasks: u32,
    ) -> Result<Vec<TaskSuggestionResult>> {
        let (plexo_engine, _member_id) = extract_context(ctx)?;

        let task_id = task_id.parse::<uuid::Uuid>()?;

        let suggestions = plexo_engine
            .auto_suggestions_engine
            .subdivide_task(task_id, subtasks)
            .await
            .unwrap();

        // let raw_suggestion = plexo_engine
        //     .auto_suggestions_engine
        //     .get_suggestions(TaskSuggestion {
        //         title: task.title,
        //         description: task.description,
        //         status: task.status,
        //         priority: task.priority,
        //         due_date: task.due_date,
        //     })
        //     .await;
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

        // Ok(task_id.to_string())
        Ok(suggestions)
    }
}
