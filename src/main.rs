use async_graphql::{
    futures_util::Stream,
    futures_util::StreamExt,
    // dataloader::DataLoader,
    http::GraphiQLSource,
    EmptyMutation,
    Object,
    Schema as GraphQLSchema,
    Subscription,
};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use dotenvy::dotenv;
use lazy_static::lazy_static;
// use plexo::QueryRoot;
// use crate::entities::task;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
// use sea_orm::{
//     ActiveModelTrait, ConnectionTrait, Database, DatabaseBackend, DbBackend,
//     Schema as SeaORMSchema, Statement,
// };
use std::{env, time::Duration};

// use sea_orm::ActiveValue::{NotSet, Set};
use sqlx::postgres::PgPoolOptions;

// extern crate async_graphql;
// extern crate tokio;
// extern crate tokio_stream;

// use entities::task::Model as Task;

// mod entities;

lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("0.0.0.0:8080".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

// #[handler]
// async fn graphql_playground() -> impl IntoResponse {
//     Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
// }

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(format!("http://{}", *URL).as_str())
            .subscription_endpoint(format!("ws://{}/ws", *URL).as_str())
            .finish(),
    )
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello World!".to_string()
    }
}

struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }
    // async fn tasks(&self) -> impl Stream<Item = Task> {
    //     tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
    //         .map(|_| Task {
    //             id: "1".to_string(),
    //             title: "Task 1".to_string(),
    //             created_at: None,
    //             updated_at: None,
    //             description: None,
    //         })
    // }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::INFO)
    //     .with_test_writer()
    //     .init();

    // let db = Database::connect(&*DATABASE_URL)
    //     .await
    //     .expect("Fail to initialize database connection");

    // db.
    // println!("{:?}", db.get_database_backend());
    // let res = db
    //     .execute(Statement::from_string(
    //         DatabaseBackend::Postgres,
    //         "LISTEN myevent;".to_owned(),
    //     ))
    //     .await
    //     .unwrap();

    // let db_postgres = DbBackend::Postgres;
    // let schema = SeaORMSchema::new(db_postgres);

    // println!(
    //     "{}",
    //     db.get_database_backend()
    //         .build(&schema.create_table_from_entity(task::Entity))
    //         .clone()
    //         .sql
    // );

    // let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
    //     OrmDataloader {
    //         db: database.clone(),
    //     },
    //     tokio::spawn,
    // );

    // let t = task::ActiveModel {
    //     title: Set("Task 1".to_owned()),
    //     ..Default::default()
    // };

    // t.insert(&db).await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .unwrap();

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(420_i64)
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();

    // println!("{:?}", row);

    // let task_id = "1".to_string();

    let projects = sqlx::query!("SELECT id, name FROM projects")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", projects);

    let schema = GraphQLSchema::build(QueryRoot, EmptyMutation, SubscriptionRoot);
    // .data(database)
    // .data(orm_dataloader);
    // .finish();

    // if let Some(depth) = *DEPTH_LIMIT {
    //     schema = schema.limit_depth(depth);
    // }
    // if let Some(complexity) = *COMPLEXITY_LIMIT {
    //     schema = schema.limit_complexity(complexity);
    // }

    let schema = schema.finish();

    let app = Route::new()
        .at(
            ENDPOINT.to_owned(),
            get(graphiql).post(GraphQL::new(schema.clone())),
        )
        .at("/ws", get(GraphQLSubscription::new(schema)));

    println!("Visit GraphQL Playground at http://{}", *URL);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");
}
