// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::{IntoParams, IntoResponses, ToSchema};

// API
// - Input
#[derive(Deserialize, IntoParams, ToSchema)]
pub struct NagoyaCheckDataCC {
    // TODO: Use additional validation
    // TODO: Add data for registered collection
    // TODO: Add data for Certificates
    pub(crate) probe_country: String,
}

// TODO: Find out whether there is a proper way to do this / access the data directly
#[derive(Deserialize, ToSchema)]
pub struct NagoyaCheckDataGeo {
    pub(crate) coordinates: Coordinates,
}

#[derive(Deserialize, ToSchema)]
pub struct Coordinates {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}
// - Output
#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
pub struct NagoyaResponse {
    pub(crate) check_result: bool,
}

#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
pub struct GenericResponse {
    pub(crate) message: String,
}

// External Requests
#[derive(Deserialize, PartialEq, Hash, Eq, Debug)]
pub struct NagoyaCountryInfo {
    pub(crate) code2: String,
    pub(crate) code3: String,
    //pub(crate) nagoya_info: NagoyaTreatyInfo,
    //#[serde(flatten)]
    pub(crate) treaties: Treaties,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Debug)]
pub struct Treaties {
    //#[serde(rename = "XXVII8b", flatten)]
    #[serde(rename = "XXVII8b")]
    pub(crate) nagoya: Treaty,
}

#[derive(Deserialize, PartialEq, Hash, Eq, Debug)]
pub struct Treaty {
    #[serde(rename = "party")]
    pub(crate) party_date: Option<String>,
}

// Internal
#[derive(Deserialize, Clone)]
pub struct ImplementingCountries {
    pub(crate) countries: HashSet<String>,
}

impl FromRef<AppState> for ImplementingCountries {
    fn from_ref(app_state: &AppState) -> ImplementingCountries {
        app_state.implementing_countries.clone()
    }
}

#[derive(Deserialize)]
pub struct NominatimAddress {
    pub(crate) country_code: String,
}
#[derive(Deserialize)]
pub struct NominatimResponse {
    pub(crate) address: NominatimAddress,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub nominatim_host: String,
    pub server_host: String,
    pub server_port: u16,
}

impl FromRef<AppState> for Config {
    fn from_ref(app_state: &AppState) -> Config {
        app_state.config.clone()
    }
}
#[derive(Clone, Deserialize)]
pub struct AppState {
    pub implementing_countries: ImplementingCountries,
    pub config: Config,
}
//pub struct Cache<T> {
//    // Can run longer
//    // TODO: Use monotonic clock
//    timestamp: u64,
//    data: T,
//}
