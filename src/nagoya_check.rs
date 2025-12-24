use crate::external_data::fetch_country_code_by_coordinates;
use crate::models::{Config, Coordinates, ImplementingCountries, NagoyaResponse};
use axum::Json;
use std::error::Error;

async fn is_probe_in_implementing_country(
    implementing_countries: &ImplementingCountries,
    probe_country: &str,
) -> Result<bool, Box<dyn Error>> {
    // Check whether probe country is in list of implementing countries
    Ok(implementing_countries.countries.contains(probe_country))
}

pub async fn nagoya_check_cc(
    probe_country: String,
    implementing_countries: &ImplementingCountries,
) -> Result<Json<NagoyaResponse>, Box<dyn Error>> {
    Ok(Json(NagoyaResponse {
        check_result: is_probe_in_implementing_country(implementing_countries, &probe_country)
            .await?,
    }))
}

pub async fn nagoya_check_geo(
    coordinates: Coordinates,
    implementing_countries: &ImplementingCountries,
    config: &Config, // Host meaningless here, so unpacked just before use
) -> Result<Json<NagoyaResponse>, Box<dyn Error>> {
    nagoya_check_cc(
        // TODO: Add error handling for failing fetch
        fetch_country_code_by_coordinates(config, coordinates).await,
        implementing_countries,
    )
    .await
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
