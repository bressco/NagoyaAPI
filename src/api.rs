// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::ApiDoc;
use crate::models::{
    AppState, GenericResponse, ImplementingCountries, NagoyaCheckDataCC, NagoyaCheckDataGeo,
    NagoyaError, NagoyaResponse,
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
        (status = 200, description = "Result of the compliance check", body = NagoyaResponse),
        (status = 422, description = "Could not process input, possibly illegal country code"),
        (status = 502)
    )
)]
pub async fn nagoya_check_country_code(
    State(implementing_countries): State<ImplementingCountries>,
    Json(payload): Json<NagoyaCheckDataCC>,
) -> Result<Json<NagoyaResponse>, axum::http::StatusCode> {
    match nagoya_check_cc(payload.probe_country, &implementing_countries).await {
        Ok(res) => Ok(res),
        Err(NagoyaError::MalformedCountryCode) => Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/nagoya_check_geo",
    request_body = NagoyaCheckDataGeo,
    responses(
        (status = 200, description ="Result of the compliance check", body = NagoyaResponse),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway")
    )
)]
pub async fn nagoya_check_geocoordinates(
    //State(implementing_countries): State<ImplementingCountries>,
    //State(config): State<Config>,
    State(mut state): State<AppState>,
    Json(payload): Json<NagoyaCheckDataGeo>,
) -> Result<Json<NagoyaResponse>, axum::http::StatusCode> {
    // TODO: Explicit error handling via match here?
    // TODO: More granular error response, e.g. bc upstream failed
    //Ok(nagoya_check_geo(payload.coordinates, &implementing_countries, &config).await?)
    match nagoya_check_geo(
        payload.coordinates,
        &state.implementing_countries().await.clone(),
        &state.config,
    )
    .await
    {
        Ok(res) => Ok(res),
        Err(NagoyaError::UnresolvableCoordinates) => Err(axum::http::StatusCode::BAD_GATEWAY),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
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
