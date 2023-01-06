use std::env;

use async_graphql::{http::GraphiQLSource, EmptyMutation, Schema};
use async_graphql_poem::{GraphQL, GraphQLSubscription};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use plexo::{
    entities::task::TaskBuilder,
    graphql::{query::QueryRoot, subscription::SubscriptionRoot},
};
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};

use sqlx::postgres::PgPoolOptions;

lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("0.0.0.0:8080".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    // static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
    //     data.parse().expect("DEPTH_LIMIT is not a number")
    // ));
    // static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
    //     .map_or(None, |data| {
    //         Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
    //     });
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(format!("http://{}", *URL).as_str())
            .subscription_endpoint(format!("ws://{}/ws", *URL).as_str())
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .unwrap();

    let projects = sqlx::query!("SELECT id, name, created_at FROM projects")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", projects);

    let t = TaskBuilder::new("Task 1".to_string())
        .insert(&pool)
        .await
        .unwrap();

    println!("{:?}", t);

    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot);

    let schema = schema.data("data").finish();

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
