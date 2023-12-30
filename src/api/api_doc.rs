use url_builder::URLBuilder;
use utoipa::OpenApi;
use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};

use config::Auth0Config;
use contracts::dto::error_response_dto::ErrorResponseDto;
use contracts::dto::message_dto::MessageDto;
use contracts::error_code::error_code::ErrorCode;
use contracts::permissions::message_permissions::MessagePermissions;

use crate::api::messages::handlers;

#[derive(OpenApi)]
#[openapi(
paths(
handlers::admin,
handlers::protected,
handlers::public,
),
components(
schemas(ErrorResponseDto, MessageDto, ErrorCode)
),
tags(
(name = "todo", description = "Todo management endpoints.")
),
modifiers(& SecurityAddon)
)]
pub struct ApiDoc;


struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let auth0_config = Auth0Config::default();
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "OAuth2",
            SecurityScheme::OAuth2(
                OAuth2::new([
                    Flow::AuthorizationCode(
                        AuthorizationCode::new(
                            get_authorization_url(&auth0_config),
                            get_token_url(&auth0_config),
                            Scopes::from_iter([
                                (MessagePermissions::ReadAdminMessages.to_string().as_str(), "Read admin message"),
                            ]),
                        )
                    )
                ])
            ),
        );
        log::info!("Authorization URL: {}", get_authorization_url(&auth0_config));
        log::info!("Token URL: {}", get_token_url(&auth0_config));
    }
}

fn get_token_url(auth0_config: &Auth0Config) -> String {
    let mut url_builder = URLBuilder::new();
    url_builder.set_protocol("https");
    url_builder.set_host(auth0_config.domain.as_str());
    url_builder.add_route("oauth");
    url_builder.add_route("token");
    url_builder.add_param("audience", auth0_config.audience.as_str());
    url_builder.build()
}

fn get_authorization_url(auth0_config: &Auth0Config) -> String {
    let mut url_builder = URLBuilder::new();
    url_builder.set_protocol("https");
    url_builder.set_host(auth0_config.domain.as_str());
    url_builder.add_route("authorize");
    url_builder.add_param("audience", auth0_config.audience.as_str());
    url_builder.build()
}

pub fn create_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
