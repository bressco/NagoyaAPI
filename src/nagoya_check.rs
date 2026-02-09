// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::external_data::fetch_country_code_by_coordinates;
use crate::models::{Config, Coordinates, ImplementingCountries, NagoyaResponse};
use axum::Json;
use std::error::Error;
use tracing::{Level, event, instrument, span};

#[instrument(skip(implementing_countries))]
pub async fn nagoya_check_cc(
    probe_country: String,
    implementing_countries: &ImplementingCountries,
) -> Result<Json<NagoyaResponse>, Box<dyn Error + Send + Sync>> {
    let span = span!(Level::DEBUG, "Lookup via Country Code");
    let _enter = span.enter();
    Ok(Json(NagoyaResponse {
        check_result: is_probe_in_implementing_country(implementing_countries, &probe_country)
            .await?,
    }))
}

#[instrument(skip(implementing_countries))]
pub async fn nagoya_check_geo(
    coordinates: Coordinates,
    implementing_countries: &ImplementingCountries,
    config: &Config, // Host meaningless here, so unpacked just before use
) -> Result<Json<NagoyaResponse>, Box<dyn Error + Send + Sync>> {
    let span = span!(Level::DEBUG, "Lookup via Geocoordinates");
    let _enter = span.enter();
    nagoya_check_cc(
        // TODO: Add error handling for failing fetch
        fetch_country_code_by_coordinates(config, coordinates).await?,
        implementing_countries,
    )
    .await
}

#[instrument]
async fn is_probe_in_implementing_country(
    implementing_countries: &ImplementingCountries,
    probe_country: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    // Check whether probe country is in list of implementing countries
    event!(
        Level::DEBUG,
        "Checking whether \"{}\" is implementing the Nagoya Protocol",
        &probe_country
    );
    let probe_country_code3: &str;
    if probe_country.len() == 3 {
        probe_country_code3 = rust_iso3166::from_alpha3(&probe_country.to_uppercase())
            .unwrap()
            .alpha3;
    } else if probe_country.len() == 2 {
        //TODO: Add error handling (e.g. if correct length, but country code incorrect
        probe_country_code3 = rust_iso3166::from_alpha2(&probe_country.to_uppercase())
            .unwrap()
            .alpha3;
    } else {
        // TODO: Fix error handling
        panic!("Invalid country code")
    }
    Ok(implementing_countries
        .countries
        .contains(&probe_country_code3.to_uppercase()))
}

#[tokio::test]
#[allow(clippy::needless_borrow)]
async fn test_probe_in_implementing_country() {
    use std::collections::HashSet;
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

    assert!(
        is_probe_in_implementing_country(&data_included, &probe)
            .await
            .unwrap()
    );
    assert!(
        is_probe_in_implementing_country(&data_included_single, &probe)
            .await
            .unwrap()
    );
    assert!(
        !is_probe_in_implementing_country(&data_not_included, &probe)
            .await
            .unwrap()
    );
    assert!(
        !is_probe_in_implementing_country(&data_empty, &probe)
            .await
            .unwrap()
    );
}
