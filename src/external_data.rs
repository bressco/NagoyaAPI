use crate::models::{
    Config, Coordinates, ImplementingCountries, NagoyaCountryInfo, NominatimResponse,
};
use std::collections::HashSet;
use std::error::Error;

async fn fetch_absch_treaty_info() -> String {
    // Creates a new client on each call. As this call should happen rarely due to caching, this
    // should be ok
    // Just gets the json
    let absch_json = reqwest::get("https://api.cbd.int/api/v2013/countries/").await;

    // Error handling here instead of passing

    match absch_json {
        Ok(x) => {
            match x.error_for_status() {
                Ok(x) => x.text().await.unwrap(),
                // Error on server side
                Err(e) => {
                    // TODO: Handle this cleaner
                    panic!("Server returned an error: {}", e.status().unwrap())
                }
            }
        }
        // Error if for different reasons a connection cannot be established
        // TODO: Act differently based on server error, e.g. try again with temp errors
        Err(e) => {
            panic!("Could not fetch data from ABSCH: {}", e)
        }
    }

    // TODO: Make several attempts instead of just once and then giving up
    // If a cache is used, that could be used instead, also to steer the retry time by resetting
    // the stale "timer". But for now this should be enough
    // At least, if the fetching fails repeatedly, maybe configurable in .env
}

pub async fn fetch_country_code_by_coordinates(
    config: &Config,
    coordinates: Coordinates,
) -> String {
    let request = format!(
        "{}{}?lat={}&lon={}&json",
        //env_map.get("NOMINATIM_HOST").unwrap(),
        config.nominatim_host,
        "/reverse",
        coordinates.latitude,
        coordinates.longitude
    );

    let nominatim_json: NominatimResponse =
        reqwest::get(request).await.unwrap().json().await.unwrap();
    nominatim_json.address.country_code
}

fn get_nagoya_treaty_info(absch_json: &str) -> Result<HashSet<NagoyaCountryInfo>, Box<dyn Error>> {
    let v: HashSet<NagoyaCountryInfo> = serde_json::from_str(absch_json)?;
    Ok(v)
}

pub async fn get_implementing_countries() -> Result<ImplementingCountries, Box<dyn Error>> {
    // TODO: Use Caching
    // TODO: Instead of strings, use the data model provided by the iso3166 crate (or return a fitting error)

    // Get JSON from ABSCH (if not in cache; cache duration in config)
    let nagoya_country_info = get_nagoya_treaty_info(&fetch_absch_treaty_info().await)?;

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
