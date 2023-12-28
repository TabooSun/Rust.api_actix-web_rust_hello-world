use actix_web::{
    dev::ServiceResponse,
    http::StatusCode,
    HttpResponse,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    Result,
};

use contracts::dto::error_response_dto::ErrorResponseDto;
use contracts::error_code::error_code::ErrorCode;

pub fn err_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(StatusCode::NOT_FOUND, not_found)
}


fn internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::InternalServerError().json(ErrorResponseDto {
        error_code: ErrorCode::InternalServerError,
        error_description: None,
        message: "Internal server error".to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });
    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.map_into_right_body()),
    ))
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::NotFound().json(ErrorResponseDto {
        error_code: ErrorCode::NotFound,
        error_description: None,
        message: "Not Found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    });
    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.map_into_right_body()),
    ))
}
