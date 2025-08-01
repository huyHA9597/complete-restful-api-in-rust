pub mod config;
pub mod db;
pub mod dtos;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod model;
pub mod open_api;
pub mod utils;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::open_api::ApiDoc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        openssl_probe::init_openssl_env_vars();
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully."),
        Err(e) => eprintln!("Error executing migrations: {e}"),
    };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_origin("https://rust.codevoweb.com")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .service(handlers::auth::auth_scope())
            .service(handlers::users::users_scope())
            .service(handlers::healthcheck::healthcheck_scope())
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}
