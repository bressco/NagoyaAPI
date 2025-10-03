use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use utoipa::{IntoParams, IntoResponses, OpenApi, ToSchema};
//use utoipa_swagger_ui;
//use validator::{Validate, ValidationError};

#[derive(OpenApi)]
#[openapi(paths(openapi, nagoya_check, health_check))]
struct ApiDoc;
// get countries
#[derive(Deserialize, Clone)]
struct ImplementingCountries {
    countries: HashSet<String>,
}

#[derive(Deserialize, IntoParams)]
struct NagoyaCheckData {
    // TODO: Use enum instead of string? E.g. crate iso3166
    // TODO: Use additional validation
    researcher_affils: HashSet<String>,
    //#[validate(length(min = 3, max = 3))]
    probe_country: String,
}

#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
struct NagoyaResponse {
    check_result: bool,
    status_code: u16,
}

#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
struct GenericResponse {
    message: String,
    status_code: u16,
}

fn get_implementing_countries() -> Result<ImplementingCountries, Box<dyn Error>> {
    let v: ImplementingCountries =
        serde_json::from_str(include_str!("../assets/nagoya_countries.json"))?;
    Ok(v)
}

/// Checks whether the probe is from a country implementing Nagoya Measures. If so, the Result
/// contains true.
///
/// # Arguments
///
/// * `implementing_countries`: Countries implementing Nagoya measures.
/// * `probe_country`: Country from where the probe was or is to be extracted
///
/// returns: Result<bool, Box<dyn Error, Global>>
///
/// # Examples
///
/// ```
///
/// ```
async fn is_probe_in_implementing_country(
    implementing_countries: &ImplementingCountries,
    probe_country: &str,
) -> Result<bool, Box<dyn Error>> {
    // Check whether probe country is in list of implementing countries
    Ok(implementing_countries.countries.contains(probe_country))
}

/// Checks whether at least one of the researchers is from the same country as the probe.
/// Result contains true, if at least one of the researchers is from the country of the probe.
///
/// # Arguments
///
/// * `affils`: Country Affiliations of the researchers
/// * `probe_country`: Country from where the probe was or is to be extracted
///
/// returns: Result<bool, Box<dyn Error, Global>>
///
async fn are_affils_from_probe_country(
    affils: &HashSet<String>,
    probe_country: &str,
) -> Result<bool, Box<dyn Error>> {
    // Check whether someone is from the country of the probe's origin.
    // Substract the country from the HS and compare length
    Ok(affils.contains(probe_country))
}

#[utoipa::path(
    post,
    path = "/nagoya_check",
    responses(
    (status = 200, description = "Result of the compliance check", body = NagoyaResponse)
    )
)]
async fn nagoya_check(
    Json(payload): Json<NagoyaCheckData>,
    implementing_countries: ImplementingCountries,
) -> Json<NagoyaResponse> {
    let probe_bool: bool =
        is_probe_in_implementing_country(&implementing_countries, &payload.probe_country)
            .await
            .unwrap();
    let affils_bool: bool =
        are_affils_from_probe_country(&payload.researcher_affils, &payload.probe_country)
            .await
            .unwrap();

    Json(NagoyaResponse {
        check_result: probe_bool & affils_bool,
        status_code: 200,
    })
}

// Wrapper to ease testing of the main functionality
async fn nagoya_check_wrapper(
    State(implementing_countries): State<ImplementingCountries>,
    Json(payload): Json<NagoyaCheckData>,
) -> Json<NagoyaResponse> {
    nagoya_check(Json(payload), implementing_countries).await
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description ="JSON file", body=GenericResponse)
    )
)]
async fn health_check() -> Json<GenericResponse> {
    Json(GenericResponse {
        message: String::from("NagoyaAPI is running"),
        status_code: 200,
    })
}

#[utoipa::path(
    get,
    path = "/openapi.json",
    responses(
        (status = 200, description ="JSON file", body=())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    // Load env
    dotenv().ok();

    // Load List of Countries implementing measures according to the Nagoya Protocol
    let implementing_countries = get_implementing_countries().unwrap();

    let server_address = dotenvy::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let service_port = dotenvy::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Please select a valid port number of between 0 and 65535");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_address, service_port))
        .await
        .unwrap();

    let app = Router::new()
        .route("/nagoya_check", post(nagoya_check_wrapper))
        .route("/openapi.json", get(openapi))
        .route("/health", get(health_check))
        //.merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .with_state(implementing_countries);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_are_affils_from_probe_country() {
        let data_included = HashSet::from_iter(vec!["AUS".to_string(), "DEU".to_string()]);
        let data_not_included = HashSet::from_iter(vec!["DEU".to_string()]);
        let data_empty = HashSet::from_iter(vec!["".to_string()]);
        let probe: &str = "AUS";

        assert_eq!(
            are_affils_from_probe_country(&data_included, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            are_affils_from_probe_country(&data_not_included, &probe)
                .await
                .unwrap(),
            false
        );
        assert_eq!(
            are_affils_from_probe_country(&data_empty, &probe)
                .await
                .unwrap(),
            false
        );
    }

    #[tokio::test]
    async fn test_probe_in_implementing_country() {
        let data_included = ImplementingCountries {
            countries: HashSet::from_iter(vec![String::from("DEU"), String::from("AUS")]),
        };
        let data_included_single = ImplementingCountries {
            countries: HashSet::from_iter(vec![String::from("DEU")]),
        };
        let data_not_included = ImplementingCountries {
            countries: HashSet::from_iter(vec![String::from("AFG")]),
        };
        let data_empty = ImplementingCountries {
            countries: HashSet::new(),
        };
        let probe = "DEU";

        assert_eq!(
            is_probe_in_implementing_country(&data_included, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_included_single, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_not_included, &probe)
                .await
                .unwrap(),
            false
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_empty, &probe)
                .await
                .unwrap(),
            false
        );
    }
}
