use dotenvy::dotenv;
use plexo::{
    auth::{
        core::{
            email_basic_login_handler, github_callback_handler, github_sign_in_handler,
            logout_handler,
        },
        engine::AuthEngine,
    },
    config::{
        DATABASE_URL, DOMAIN, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL,
        JWT_ACCESS_TOKEN_SECRET, STATIC_PAGE_ENABLED, URL,
    },
    handlers::{graphiq_handler, index_handler, ws_switch_handler},
    statics::StaticServer,
    system::{core::Engine, prelude::Prelude, schema::GraphQLSchema},
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let plexo_engine = Engine::new(
        PgPoolOptions::new()
            .max_connections(3)
            .connect(&DATABASE_URL)
            .await
            .unwrap(),
        AuthEngine::new(
            // TODO: That's horrible, fix it
            (*JWT_ACCESS_TOKEN_SECRET).to_string(),
            (*JWT_ACCESS_TOKEN_SECRET).to_string(),
            (*GITHUB_CLIENT_ID).to_owned(),
            (*GITHUB_CLIENT_SECRET).to_owned(),
            Some((*GITHUB_REDIRECT_URL).to_owned()),
        ),
    );

    plexo_engine.prelude().await;

    let schema = plexo_engine.graphql_api_schema();

    let static_page_root_path = "plexo-platform/out".to_string();

    let static_page =
        StaticServer::new(static_page_root_path, plexo_engine.clone()).index_file("index.html");

    let mut app = Route::new()
        // .nest("/", static_page)
        // Non authenticated routes
        .at("/auth/email/login", post(email_basic_login_handler))
        // .at("/auth/email/register", post(email_basic_register_handler))
        //
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        //
        .at("/auth/logout", get(logout_handler))
        //
        .at("/playground", get(graphiq_handler))
        .at("/graphql", post(index_handler))
        .at("/graphql/ws", get(ws_switch_handler));

    if *STATIC_PAGE_ENABLED {
        println!("Static page enabled");
        app = app.nest("/", static_page)
    }

    let app = app
        .with(
            Cors::new().allow_credentials(true), // .expose_header("Set-Cookie"),
        )
        .data(schema)
        .data(plexo_engine.clone());

    println!("Visit GraphQL Playground at {}/playground", *DOMAIN);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");
}
