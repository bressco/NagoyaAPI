// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::external_data::fetch_country_code_by_coordinates;
use crate::models::{Config, Coordinates, ImplementingCountries, NagoyaError, NagoyaResponse};
use axum::Json;
use tracing::{Level, event, instrument, span};

#[instrument(skip(implementing_countries))]
pub async fn nagoya_check_cc(
    probe_country: String,
    implementing_countries: &ImplementingCountries,
    //) -> Result<Json<NagoyaResponse>, Box<dyn Error + Send + Sync>> {
) -> Result<Json<crate::models::NagoyaResponse>, NagoyaError> {
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
                     //) -> Result<Json<NagoyaResponse>, Box<dyn Error + Send + Sync>> {
) -> Result<Json<NagoyaResponse>, NagoyaError> {
    let span = span!(Level::DEBUG, "Lookup via Geocoordinates");
    let _enter = span.enter();
    nagoya_check_cc(
        fetch_country_code_by_coordinates(config, coordinates)
            .await
            // Let's stick with this instead of differentiating between malformed coordinates and
            // external service not reachable. Either way the external service tells us both
            // If reverse lookup is not possible, nominatim returns {"error":"Unable to geocode"}
            // with Status 200
            .map_err(|_| NagoyaError::UnresolvableCoordinates)?,
        implementing_countries,
    )
    .await
}

#[instrument]
async fn is_probe_in_implementing_country(
    implementing_countries: &ImplementingCountries,
    probe_country: &str,
    //) -> Result<bool, Box<dyn Error + Send + Sync>> {
) -> Result<bool, NagoyaError> {
    // Check whether probe country is in list of implementing countries
    event!(
        Level::DEBUG,
        "Checking whether \"{}\" is implementing the Nagoya Protocol",
        &probe_country
    );
    let probe_country_code3: &str;
    if probe_country.len() == 3 {
        probe_country_code3 = rust_iso3166::from_alpha3(&probe_country.to_uppercase())
            .ok_or(NagoyaError::MalformedCountryCode)?
            .alpha3;
    } else if probe_country.len() == 2 {
        //TODO: Add error handling (e.g. if correct length, but country code incorrect
        probe_country_code3 = rust_iso3166::from_alpha2(&probe_country.to_uppercase())
            .ok_or(NagoyaError::MalformedCountryCode)?
            .alpha3;
    } else {
        // TODO: Fix error handling
        panic!("Invalid country code")
    }
    Ok(implementing_countries
        .countries
        .contains(&probe_country_code3.to_uppercase()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[tokio::test]
    #[allow(clippy::needless_borrow)]
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

    #[tokio::test]
    #[allow(clippy::needless_borrow)]
    async fn test_malformed_probe() {
        // Test is about handling of an illegal country code, so the contents of the data do not matter
        let data_empty = ImplementingCountries {
            countries: HashSet::new(),
        };

        let malformed_probe = "XYZ";
        assert_eq!(
            is_probe_in_implementing_country(&data_empty, &malformed_probe)
                .await
                .unwrap_err(),
            //*is_probe_in_implementing_country(&data_empty, &malformed_probe)
            //    .await
            //    .unwrap_err()
            //    .downcast::<NagoyaError>()
            //    .unwrap(),
            NagoyaError::MalformedCountryCode
        )
    }
}
