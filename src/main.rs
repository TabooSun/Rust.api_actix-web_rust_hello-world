use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

mod api;
mod config;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
    paths(
    api::messages::handlers::admin,
    api::messages::handlers::protected,
    api::messages::handlers::public,
    ),
    components(
    // schemas(todo::Todo, todo::TodoUpdateRequest, todo::ErrorResponse)
    ),
    tags(
    (name = "todo", description = "Todo management endpoints.")
    ),
    // modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    let config = config::Config::default();
    let auth0_config = actix_web_auth0::extractors::Auth0Config::default();
    HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(api::routes())
    })
        .bind((std::net::Ipv4Addr::UNSPECIFIED, config.port))?
        .run()
        .await
}
