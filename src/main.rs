use std::env;

use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Schema,
};
use async_graphql_poem::{
    GraphQL, GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLSubscription,
    GraphQLWebSocket,
};

use dotenvy::dotenv;
use lazy_static::lazy_static;
use plexo::{
    auth::{
        auth::{example_auth, github_sign_in, PlexoAuthToken},
        engine::AuthEngine,
    },
    graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot},
    system::core::Engine,
};
use poem::{
    get, handler,
    http::HeaderMap,
    listener::TcpListener,
    web::{websocket::WebSocket, Data as PoemData, Html},
    EndpointExt, IntoResponse, Route, Server,
};

use serde_json::Value;
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
async fn index(
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
async fn ws(
    schema: PoemData<&Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    protocol: GraphQLProtocol,
    websocket: WebSocket,
) -> impl IntoResponse {
    let schema = schema.0.clone();
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                // connection params are used to extract the token in this fn
                .on_connection_init(on_connection_init)
                .serve()
        })
}

#[tokio::main]
async fn main() {
    // let database_url = secret_store
    //     .get("DATABASE_URL")
    //     .unwrap_or((&*DATABASE_URL.to_string()).into());

    // env::set_var("DATABASE_URL", database_url.as_str());

    dotenv().ok();

    // example_auth().await;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*DATABASE_URL)
        .await
        .unwrap();

    let auth = AuthEngine::new();

    let plexo_engine = Engine::new(pool, auth);

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(plexo_engine)
        .finish();

    let app = Route::new()
        .at(ENDPOINT.to_owned(), get(graphiql).post(index))
        .at("/ws", get(ws))
        .at("auth/sign-in/github", get(github_sign_in))
        .data(schema);

    println!("Visit GraphQL Playground at http://{}", *URL);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");
}
