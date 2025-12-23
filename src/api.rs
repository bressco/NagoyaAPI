use crate::ApiDoc;
use crate::models::{
    Config, GenericResponse, ImplementingCountries, NagoyaCheckDataCC, NagoyaCheckDataGeo,
    NagoyaResponse,
};
use crate::nagoya_check::{nagoya_check_cc, nagoya_check_geo};
use axum::Json;
use axum::extract::State;
use utoipa::OpenApi;

// Wrapper to ease testing of the main functionality
#[utoipa::path(
    post,
    path = "/nagoya_check_cc",
    request_body = NagoyaCheckDataCC,
    responses(
    (status = 200, description = "Result of the compliance check", body = NagoyaResponse)
    )
)]
pub async fn nagoya_check_country_code(
    State(implementing_countries): State<ImplementingCountries>,
    Json(payload): Json<NagoyaCheckDataCC>,
) -> Json<NagoyaResponse> {
    nagoya_check_cc(payload.probe_country, &implementing_countries).await
}

#[utoipa::path(
    post,
    path = "/nagoya_check_geo",
    request_body = NagoyaCheckDataGeo,
    responses((status=200, description ="Result of the compliance check", body = NagoyaResponse))
)]
pub async fn nagoya_check_geocoordinates(
    State(implementing_countries): State<ImplementingCountries>,
    State(config): State<Config>,
    Json(payload): Json<NagoyaCheckDataGeo>,
) -> Json<NagoyaResponse> {
    nagoya_check_geo(payload.coordinates, &implementing_countries, &config).await
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
