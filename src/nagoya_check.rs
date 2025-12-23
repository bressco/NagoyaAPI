use crate::models::{ImplementingCountries, NagoyaCheckData, NagoyaResponse};
use axum::Json;
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
async fn test_are_affils_from_probe_country() {
    //let data_included = HashSet::from_iter(vec!["AUS".to_string(), "DEU".to_string()]);
    //let data_not_included = HashSet::from_iter(vec!["DEU".to_string()]);
    //let data_empty = HashSet::from_iter(vec!["".to_string()]);
    //let probe: &str = "AUS";

    // assert!(
    //     are_affils_from_probe_country(&data_included, &probe)
    //         .await
    //         .unwrap(),
    // );
    // assert!(
    //     !are_affils_from_probe_country(&data_not_included, &probe)
    //         .await
    //         .unwrap(),
    // );
    // assert!(
    //     !are_affils_from_probe_country(&data_empty, &probe)
    //         .await
    //         .unwrap()
    // );
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
