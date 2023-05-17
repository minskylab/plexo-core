use async_graphql::{dataloader::DataLoader, Schema};
use dotenvy::dotenv;
use plexo::{
    auth::{
        core::{email_basic_login_handler, github_callback_handler, github_sign_in_handler},
        engine::AuthEngine,
    },
    config::{
        DATABASE_URL, DOMAIN, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, URL,
    },
    graphql::{mutation::MutationRoot, query::QueryRoot, subscription::SubscriptionRoot},
    handlers::{graphiq_handler, index_handler, ws_switch_handler},
    sdk::loaders::{LabelLoader, MemberLoader, ProjectLoader, TaskLoader, TeamLoader},
    statics::StaticFilesEndpointHTMLTrimmed,
    system::core::Engine,
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let plexo_engine = Engine::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&DATABASE_URL)
            .await
            .unwrap(),
        AuthEngine::new(
            &GITHUB_CLIENT_ID,
            &GITHUB_CLIENT_SECRET,
            &GITHUB_REDIRECT_URL,
        ),
    );

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(plexo_engine.clone()) // TODO: Optimize this
        .data(DataLoader::new(
            TaskLoader::new(plexo_engine.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            ProjectLoader::new(plexo_engine.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            LabelLoader::new(plexo_engine.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            MemberLoader::new(plexo_engine.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            TeamLoader::new(plexo_engine.clone()),
            tokio::spawn,
        ))
        .finish();

    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpointHTMLTrimmed::new("plexo-platform/out", plexo_engine.clone())
                .index_file("index.html"),
        )
        // Non authenticated routes
        .at("/auth/email", get(email_basic_login_handler))
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        //
        .at("/playground", get(graphiq_handler))
        // Authenticated routes
        // .at("/auth/refresh", get(refresh_token_handler))
        .at("/graphql", post(index_handler))
        .at("/graphql/ws", get(ws_switch_handler))
        .with(Cors::new())
        .data(schema)
        .data(plexo_engine.clone());

    println!("Visit GraphQL Playground at {}/playground", *DOMAIN);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");
}
