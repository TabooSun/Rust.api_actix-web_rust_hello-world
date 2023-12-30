use actix_web::{App, HttpServer};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::{oauth, SwaggerUi};

use crate::api::api_doc::create_openapi;

mod api;

mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let openapi = create_openapi();

    let config = config::Config::default();
    let auth0_config = config::Auth0Config::default();
    HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            // http://192.168.1.153:6060/redoc
            .service(Redoc::with_url("/redoc", openapi.clone()))
            // http://192.168.1.153:6060/swagger-ui/
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
                    .oauth(
                        oauth::Config::new()
                            .client_id(&auth0_config.client_id)
                            .client_secret(&auth0_config.client_secret)
                    ),
            )
            // There is no need to create RapiDoc::with_openapi because the OpenApi is served
            // via SwaggerUi instead we only make rapidoc to point to the existing doc.
            // http://192.168.1.153:6060/rapidoc
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(api::routes())
    })
        .bind((std::net::Ipv4Addr::UNSPECIFIED, config.port))?
        .run()
        .await
}
