use std::{collections::HashSet, future::Future, pin::Pin};

use actix_web::{
    Error,
    error::ResponseError,
    FromRequest, http::{StatusCode, Uri}, HttpResponse,
};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth, headers::www_authenticate::bearer::Bearer,
};
use jsonwebtoken::{
    Algorithm, decode,
    decode_header,
    DecodingKey, jwk::{AlgorithmParameters, JwkSet}, Validation,
};
use serde::Deserialize;

use crate::types::ErrorMessage;

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        envy::prefixed("AUTH0_")
            .from_env()
            .expect("Provide missing environment variables for Auth0Client")
    }
}

#[derive(Debug, thiserror::Error)]
enum ClientError {
    #[error("authentication")]
    Authentication(#[from] actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[error("decode")]
    Decode(#[from] jsonwebtoken::errors::Error),
    #[error("not_found")]
    NotFound(String),
    #[error("unsupported_algorithm")]
    UnsupportedAlgorithm(AlgorithmParameters),
    #[error("Get jwks.json error")]
    RequestJwksError(awc::error::SendRequestError),
    #[error("Parse jwks.json error")]
    ParseJwksError(awc::error::JsonPayloadError),
}

impl ResponseError for ClientError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgorithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algorithm expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
            ClientError::RequestJwksError(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some("Failed to get jwks.json".to_string()),
                message: "Bad credentials".to_string(),
            }),
            ClientError::ParseJwksError(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some("Failed to parse jwks.json".to_string()),
                message: "Bad credentials".to_string(),
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    permissions: Option<HashSet<String>>,
}

impl Claims {
    pub fn validate_permissions(&self, required_permissions: &HashSet<String>) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| permissions.is_superset(required_permissions))
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let config = req.app_data::<Auth0Config>().unwrap().clone();
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor.await?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("kid not found in token header".to_string())
            })?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = awc::Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .unwrap(),
                )
                .send()
                .await
                .map_err(ClientError::RequestJwksError)?
                .json()
                .await
                .map_err(ClientError::ParseJwksError)?;
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;
            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;
                    Ok(token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgorithm(algorithm).into()),
            }
        })
    }
}
