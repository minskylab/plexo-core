// use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, Tags, ApiResponse};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::sdk::project::Project;

use crate::sdk::task::Task;

#[derive(Tags)]
enum ApiTags {
    /// Operations about tasks
    Task,
    /// Operations about members
    Member,
    /// Operations about projects
    Project,
    /// Operations about teams
    Team,
}

#[derive(Default)]
pub struct Api {
    pub tasks: Mutex<Vec<Task>>
}



#[OpenApi]
impl Api {
    // #[oai(path = "/hello", method = "get", operation_id = "hello")]
    // async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
    //     match name.0 {
    //         Some(name) => PlainText(format!("hello, {}!", name)),
    //         None => PlainText("hello!".to_string()),
    //     }
    // }

    #[oai(path = "/tasks", method = "post", tag = "ApiTags::Task", operation_id = "create_task")]
    async fn create_task(&self, task: Json<Task>) -> CreateTaskResponse {
        let mut users = self.tasks.lock().await;
        users.insert(0, task.0.clone());

        CreateTaskResponse::Ok(Json(task.0))
    }

    #[oai(path = "/tasks", method = "get", tag = "ApiTags::Task", operation_id = "list_tasks")]
    async fn list_tasks(&self) -> ListTasksResponse {
        let users = self.tasks.lock().await;
        ListTasksResponse::Ok(Json(users.clone()))
    }

    #[oai(path = "/tasks/:id", method = "get", tag = "ApiTags::Task", operation_id = "get_task")]
    async fn get_task(&self, id: Path<String>) -> GetTaskResponse {
        // let users = self.tasks.lock().await;
        // let task = users.iter().find(|task| task.id == Uuid::from_str(id.0.as_str()));

        // match task {
        //     Some(task) => GetTaskResponse::Ok(Json(task.clone())),
        //     None => GetTaskResponse::NotFound,
        // }

        GetTaskResponse::NotFound
    }

    #[oai(path = "/tasks/:id", method = "put", tag = "ApiTags::Task", operation_id = "update_task")]
    async fn update_task(&self, id: Path<String>, task: Json<Task>) -> GetTaskResponse {
        // let mut users = self.tasks.lock().await;
        // let task = users.iter_mut().find(|task| task.id == id.0.into());
        //
        // match task {
        //     Some(task) => {
        //         *task = task.clone();
        //         GetTaskResponse::Ok(Json(task.clone()))
        //     },
        //     None => GetTaskResponse::NotFound,
        // }

        GetTaskResponse::NotFound
    }

    #[oai(path = "/tasks/:id", method = "delete", tag = "ApiTags::Task", operation_id = "delete_task")]
    async fn delete_task(&self, id: Path<String>) -> GetTaskResponse {
        // let mut users = self.tasks.lock().await;
        // let task = users.iter().find(|task| task.id == id.0.into());

        // match task {
        //     Some(task) => {
        //         // users.remove_item(task);
        //         GetTaskResponse::Ok(Json(task.clone()))
        //     },
        //     None => GetTaskResponse::NotFound,
        // }

        GetTaskResponse::NotFound
    }

    #[oai(path = "/projects", method = "post", tag = "ApiTags::Project", operation_id = "create_project")]
    async fn create_project(&self, task: Json<Project>) -> CreateProjectResponse {
        // let mut users = self.tasks.lock().await;
        // users.insert(0, task.0.clone());

        CreateProjectResponse::Ok(Json(task.0))
    }

    #[oai(path = "/projects", method = "get", tag = "ApiTags::Project", operation_id = "list_projects")]
    async fn list_projects(&self) -> ListProjectsResponse {
        // let users = self.tasks.lock().await;
        // ListTasksResponse::Ok(Json(users.clone()))
        ListProjectsResponse::Ok(Json(vec![]))
    }
}

#[derive(ApiResponse)]
enum CreateProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum ListProjectsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Project>>),

}
#[derive(ApiResponse)]
enum CreateTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
}

#[derive(ApiResponse)]
enum ListTasksResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Task>>),
}

#[derive(ApiResponse)]
enum GetTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    #[oai(status = 404)]
    NotFound,
}