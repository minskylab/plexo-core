use async_graphql::{dataloader::DataLoader, Schema};
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
        ADMIN_EMAIL, ADMIN_NAME, ADMIN_PASSWORD, DATABASE_URL, DOMAIN, GITHUB_CLIENT_ID,
        GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, JWT_ACCESS_TOKEN_SECRET, URL,
    },
    graphql::{mutations::MutationRoot, queries::QueryRoot, subscription::SubscriptionRoot},
    handlers::{graphiq_handler, index_handler, ws_switch_handler},
    sdk::loaders::{LabelLoader, MemberLoader, ProjectLoader, TaskLoader, TeamLoader},
    statics::StaticServer,
    system::core::Engine,
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use sqlx::migrate;
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

    match migrate!().run(plexo_engine.pool.as_ref()).await {
        Ok(_) => println!("Database migration successful"),
        Err(e) => println!("Database migration failed: {:?}\n", e),
    }

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot,
    )
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

    let admin_email = ADMIN_EMAIL.to_owned();

    if plexo_engine
        .get_member_by_email(admin_email.clone())
        .await
        .is_none()
    {
        let admin_password = ADMIN_PASSWORD.to_owned();
        let admin_name = ADMIN_NAME.to_owned();

        let admin_password_hash = plexo_engine.auth.hash_password(admin_password.as_str());

        let admin_member = plexo_engine
            .create_member_from_email(admin_email.clone(), admin_name, admin_password_hash)
            .await;

        if admin_member.is_none() {
            println!("Failed to create admin member");
        } else {
            println!(
                "Admin created with email: '{}' and password: '{}'",
                admin_email, admin_password
            );
        }
    }

    let app = Route::new()
        .nest(
            "/",
            StaticServer::new("plexo-platform/out", plexo_engine.clone()).index_file("index.html"),
        )
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
