use async_graphql::Object;

use crate::sdk::task::Task;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn tasks(&self) -> Vec<Task> {
        vec![]
    }

    // async fn tasks(&self) -> Vec<Task> {
    //     let pool = PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect(&*DATABASE_URL)
    //         .await
    //         .unwrap();

    //     let task = TaskBuilder::new("Task 1".to_string())
    //         .insert(&pool)
    //         .await
    //         .unwrap();

    //     vec![task]
    // }
}
