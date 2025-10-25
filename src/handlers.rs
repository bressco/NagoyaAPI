use crate::ApiDoc;
use crate::models::{GenericResponse, ImplementingCountries, NagoyaCheckData, NagoyaResponse};
use axum::Json;
use axum::extract::State;
use utoipa::OpenApi;

// Wrapper to ease testing of the main functionality
#[utoipa::path(
    post,
    path = "/nagoya_check",
    responses(
    (status = 200, description = "Result of the compliance check", body = NagoyaResponse)
    )
)]
pub async fn nagoya_check_wrapper(
    State(implementing_countries): State<ImplementingCountries>,
    Json(payload): Json<NagoyaCheckData>,
) -> Json<NagoyaResponse> {
    crate::nagoya_check::nagoya_check(Json(payload), implementing_countries).await
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description ="JSON file", body=GenericResponse)
    )
)]
pub async fn health_check() -> Json<GenericResponse> {
    Json(GenericResponse {
        message: String::from("NagoyaAPI is running"),
    })
}

#[utoipa::path(
    get,
    path = "/openapi.json",
    responses(
        (status = 200, description ="JSON file", body=())
    )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
