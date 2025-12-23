use crate::ApiDoc;
use crate::external_data::fetch_country_code_by_coordinates;
use crate::models::{
    Config, GenericResponse, ImplementingCountries, NagoyaCheckData, NagoyaCheckDataGeo,
    NagoyaResponse,
};
use crate::nagoya_check;
use axum::Json;
use axum::extract::State;
use utoipa::OpenApi;

// Wrapper to ease testing of the main functionality
#[utoipa::path(
    post,
    path = "/nagoya_check_cc",
    responses(
    (status = 200, description = "Result of the compliance check", body = NagoyaResponse)
    )
)]
pub async fn nagoya_check_country_code(
    State(implementing_countries): State<ImplementingCountries>,
    Json(payload): Json<NagoyaCheckData>,
) -> Json<NagoyaResponse> {
    nagoya_check::nagoya_check(Json(payload), implementing_countries).await
}

#[utoipa::path(
    post,
    path = "/nagoya_check_geo",
    responses((status=200, description ="Result of the compliance check", body = NagoyaResponse))
)]
pub async fn nagoya_check_geocoordinates(
    State(implementing_countries): State<ImplementingCountries>,
    State(config): State<Config>,
    //State(AppState): State<AppState>,
    Json(payload): Json<NagoyaCheckDataGeo>,
) -> Json<NagoyaResponse> {
    nagoya_check_country_code(
        State(implementing_countries),
        Json(NagoyaCheckData {
            probe_country: fetch_country_code_by_coordinates(config, payload.coordinates).await,
        }),
    )
    .await
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
