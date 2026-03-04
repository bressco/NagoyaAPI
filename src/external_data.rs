// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::models::{
    Config, Coordinates, ImplementingCountries, NagoyaCountryInfo, NagoyaError, NominatimResponse,
};
use reqwest::Client;
use std::collections::HashSet;
use std::error::Error;
use tracing::{Level, event, instrument, span};

#[instrument]
//async fn fetch_absch_treaty_info() -> String {
async fn fetch_absch_treaty_info() -> Result<String, NagoyaError> {
    // Creates a new client on each call. As this call should happen rarely due to caching, this
    // should be ok
    // Just gets the json without parsing it
    // TODO: Make URL configurable instead of hard coding
    reqwest::get("https://api.cbd.int/api/v2013/countries/")
        .await
        .map_err(|_| NagoyaError::UnreachableExternalResource)?
        .error_for_status()
        .map_err(|_| NagoyaError::UnreachableExternalResource)?
        .text()
        .await
        .map_err(|_| NagoyaError::UnparsableExternalResponse)
}

#[instrument]
pub async fn fetch_country_code_by_coordinates(
    config: &Config,
    coordinates: Coordinates,
    //) -> Result<String, Box<dyn Error + Send + Sync>> {
) -> Result<String, NagoyaError> {
    let span = span!(Level::DEBUG, "Resolving coordinates to country code");
    let _enter = span.enter();
    let request = format!(
        "{host}{endpoint}?lat={lat}&lon={lon}&format=json",
        //env_map.get("NOMINATIM_HOST").unwrap(),
        host = config.nominatim_host,
        endpoint = "/reverse",
        lat = coordinates.latitude,
        lon = coordinates.longitude
    );

    const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = Client::builder()
        .user_agent(APP_USER_AGENT) // API requires UA for interaction
        .build()
        .map_err(|_| NagoyaError::GenericInternalServerError)?;
    let nominatim_res = client
        .get(request)
        .send()
        .await
        .map_err(|_| NagoyaError::UnreachableExternalResource)?
        .text()
        .await
        .map_err(|_| NagoyaError::UnparsableExternalResponse)?;
    let nominatim_json: NominatimResponse = serde_json::from_str(&nominatim_res)
        .map_err(|_| NagoyaError::UnparsableExternalResponse)?;
    // TODO: Handle for failed to parse
    // If reverse lookup is not possible, nominatim returns {"error":"Unable to geocode"}
    // with Status 200
    // Is Unresolvable Coordinates

    event!(
        Level::DEBUG,
        "Resolved {}, {} to \"{}\"",
        &coordinates.latitude,
        &coordinates.longitude,
        &nominatim_json.address.country_code
    );

    Ok(nominatim_json.address.country_code) // returns a code 2, needs to be converted to a code 3
}

#[instrument]
fn get_nagoya_treaty_info(absch_json: &str) -> Result<HashSet<NagoyaCountryInfo>, Box<dyn Error>> {
    let v: HashSet<NagoyaCountryInfo> = serde_json::from_str(absch_json)?;
    Ok(v)
}

#[instrument]
pub async fn get_implementing_countries() -> Result<ImplementingCountries, Box<dyn Error>> {
    // TODO: Use Caching
    // TODO: Instead of strings, use the data model provided by the iso3166 crate (or return a fitting error)

    // Get JSON from ABSCH (if not in cache; cache duration in config)
    let nagoya_country_info = get_nagoya_treaty_info(&fetch_absch_treaty_info().await?)?;

    // Get List of implementing countries from the struct. Assumed that those are the countries which
    // are party to the contract
    let code3_entries: Vec<String> = nagoya_country_info
        .iter()
        .filter(|country| country.treaties.nagoya.party_date.is_some())
        .map(|country| country.code3.clone())
        .collect();

    let countries = ImplementingCountries {
        countries: HashSet::from_iter(code3_entries),
    };

    Ok(countries)
}

#[cfg(test)]
#[allow(clippy::needless_borrow)]
mod tests {
    use super::*;
    use crate::models::{Treaties, Treaty};

    #[test]
    fn test_get_nagoya_treaty_info() {
        let testdata_json = r#"
        [
            {
                "code2": "AD",
                "code3": "AND",
                "treaties": {
                    "XXVII8":   { "party": "2015-05-05" },
                    "XXVII8a":  { "party": null },
                    "XXVII8b":  { "party": "2025-10-12" },
                    "XXVII8c":  { "party": null }
                }
            }
        ]
        "#;
        let country_data = NagoyaCountryInfo {
            code3: String::from("AND"),
            code2: String::from("AD"),
            treaties: Treaties {
                nagoya: Treaty {
                    party_date: Some(String::from("2025-10-12")),
                },
            },
        };
        assert_eq!(
            get_nagoya_treaty_info(&testdata_json).unwrap(),
            HashSet::from_iter(vec!(country_data))
        );
    }
}
