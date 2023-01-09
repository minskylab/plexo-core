use std::{env, fmt::format};

use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Schema,
};
use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};

use dotenvy::dotenv;
use lazy_static::lazy_static;
use plexo::{
    auth::{
        auth::{
            github_callback_handler, github_sign_in_handler, refresh_token_handler, PlexoAuthToken,
        },
        engine::AuthEngine,
    },
    graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot},
    system::core::Engine,
};
use poem::{
    get, handler,
    http::HeaderMap,
    listener::TcpListener,
    post,
    web::{websocket::WebSocket, Data as PoemData, Html},
    EndpointExt, IntoResponse, Route, Server,
};

use serde_json::Value;
use sqlx::postgres::PgPoolOptions;

lazy_static! {
    static ref HOST: String = env::var("HOST").unwrap_or("0.0.0.0".into());
    static ref PORT: String = env::var("PORT").unwrap_or("8080".into());
    static ref URL: String = format!("{}:{}", *HOST, *PORT);
    static ref DOMAIN: String = env::var("DOMAIN").unwrap_or(format!("http://{}", *URL));
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
}

#[handler]
async fn graphiq_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(format!("{}/graphql", *DOMAIN).as_str())
            .subscription_endpoint(format!("{}/graphql/ws", DOMAIN.replace("http", "ws")).as_str())
            .finish(),
    )
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| PlexoAuthToken(s.to_string())).ok())
}

pub async fn on_connection_init(value: Value) -> async_graphql::Result<Data> {
    match &value {
        Value::Object(map) => {
            if let Some(Value::String(token)) = map.get("Authorization") {
                let mut data = Data::default();
                data.insert(token.to_string());
                return Ok(data);
            } else {
                Err("Authorization token is required".into())
            }
        }
        _ => Err("Authorization token is required".into()),
    }
}

#[handler]
async fn index_handler(
    schema: PoemData<&Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    if let Some(token) = get_token_from_headers(headers) {
        req = req.data(token);
    }

    schema.execute(req).await.into()
}

#[handler]
async fn ws_switch_handler(
    schema: PoemData<&Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    protocol: GraphQLProtocol,
    websocket: WebSocket,
) -> impl IntoResponse {
    let schema = schema.0.clone();
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .on_connection_init(on_connection_init)
                .serve()
        })
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let plexo_engine = Engine::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&*DATABASE_URL)
            .await
            .unwrap(),
        AuthEngine::new(),
    );

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(plexo_engine.clone()) // TODO: Optimize this
        .finish();

    let app = Route::new()
        // Non authenticated routes
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        .at("/playground", get(graphiq_handler))
        // Authenticated routes
        .at("/auth/refresh", get(refresh_token_handler))
        .at("/graphql", post(index_handler))
        .at("/graphql/ws", get(ws_switch_handler))
        // .at("/", todo!()) // TODO: Serve static files
        .data(schema)
        .data(plexo_engine);

    println!("Visit GraphQL Playground at http://{}/playground", *URL);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");
}
