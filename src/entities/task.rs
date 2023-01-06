// use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

// #[derive(Clone, Debug, PartialEq)]
// pub struct Task {
//     pub id: String,
//     pub created_at: Option<PrimitiveDateTime>,
//     pub updated_at: Option<PrimitiveDateTime>,
//     pub title: String,
//     pub description: Option<String>,
// }

// pub struct TaskBuilder {
//     title: String,
//     description: Option<String>,
// }

// impl TaskBuilder {
//     pub fn new(title: String) -> Self {
//         Self {
//             title,
//             description: None,
//         }
//     }

//     pub async fn insert(&self, pool: &Pool<Postgres>) -> Result<Task, sqlx::Error> {
//         let mut tx = pool.begin().await?;

//         let new_task = sqlx::query_as!(
//             Task,
//             "
//             INSERT INTO tasks (title, description)
//             VALUES ($1, $2)
//             ",
//             self.title,
//             self.description,
//         )
//         .execute(&mut tx)
//         .await
//         .unwrap();

//         tx.commit().await.unwrap();

//         new_task.rows_affected();
//         // Ok(new_task)
//         Ok(Task {
//             id: "1".to_string(),
//             created_at: None,
//             updated_at: None,
//             title: "Task 1".to_string(),
//             description: None,
//         })
//     }
// }
