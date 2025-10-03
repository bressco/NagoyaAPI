use crate::models::ImplementingCountries;
use std::error::Error;

pub fn get_implementing_countries() -> Result<ImplementingCountries, Box<dyn Error>> {
    let v: ImplementingCountries =
        serde_json::from_str(include_str!("../assets/nagoya_countries.json"))?;
    Ok(v)
}
