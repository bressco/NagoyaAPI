use crate::models::{ImplementingCountries, NagoyaCheckData, NagoyaResponse};
use axum::Json;
use std::collections::HashSet;
use std::error::Error;

/// Checks whether the probe is from a country implementing Nagoya Measures. If so, the Result
/// contains true.
///
/// # Arguments
///
/// * `implementi pub(crate)ng_countries`: Countries implementing Nagoya measures.
/// * `probe_country`: Country from where the probe was or is to be extracted
///
/// returns: Result<bool, Box<dyn Error, Global>>
///
/// # Examples
///
/// ```
///
/// ```
pub async fn is_probe_in_implementing_country(
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
pub async fn are_affils_from_probe_country(
    affils: &HashSet<String>,
    probe_country: &str,
) -> Result<bool, Box<dyn Error>> {
    // Check whether someone is from the country of the probe's origin.
    // Substract the country from the HS and compare length
    Ok(affils.contains(probe_country))
}

pub async fn nagoya_check(
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
