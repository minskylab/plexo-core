use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Schema,
};
use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use serde_json::Value;

use crate::{
    auth::auth::PlexoAuthToken,
    config::DOMAIN,
    graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot},
};

use poem::{
    handler,
    http::HeaderMap,
    web::Html,
    web::{websocket::WebSocket, Data as PoemData},
    IntoResponse,
};

#[handler]
pub async fn graphiq_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(format!("{}/graphql", *DOMAIN).as_str())
            .subscription_endpoint(format!("{}/graphql/ws", DOMAIN.replace("http", "ws")).as_str())
            .finish(),
    )
}

#[handler]
pub async fn index_handler(
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
pub async fn ws_switch_handler(
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
                Ok(data)
            } else {
                Err("Authorization token is required".into())
            }
        }
        _ => Err("Authorization token is required".into()),
    }
}
