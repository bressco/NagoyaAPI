use crate::models::{ImplementingCountries, NagoyaCheckData, NagoyaResponse};
use axum::Json;
use std::error::Error;

pub async fn is_probe_in_implementing_country(
    implementing_countries: &ImplementingCountries,
    probe_country: &str,
) -> Result<bool, Box<dyn Error>> {
    // Check whether probe country is in list of implementing countries
    Ok(implementing_countries.countries.contains(probe_country))
}

pub async fn nagoya_check(
    Json(payload): Json<NagoyaCheckData>,
    implementing_countries: ImplementingCountries,
) -> Json<NagoyaResponse> {
    Json(NagoyaResponse {
        check_result: is_probe_in_implementing_country(
            &implementing_countries,
            &payload.probe_country,
        )
        .await
        .unwrap(),
    })
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
