// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::external_data;
use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::Instant;
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
#[derive(Clone)]
pub struct AppState {
    //pub implementing_countries: ImplementingCountries, //replace with Cache<ImplementingCountries>
    pub config: Config,
    implementing_countries: Cache<ImplementingCountries>,
}

impl AppState {
    // Returns the current version of implementing countries; fetching a new one, if needed
    async fn implementing_countries(&mut self) -> &ImplementingCountries {
        self.implementing_countries.get().await
    }
}

#[derive(Clone)]
pub struct Cache<T> {
    // Can run longer
    last_updated: Instant,
    data: T,
    ttl: Duration,
}

impl Cache<ImplementingCountries> {
    async fn is_fresh(&self) -> bool {
        return &self.last_updated.elapsed() <= &self.ttl;
    }
    async fn update(&mut self) {
        // TODO: Add proper error handling
        self.data = external_data::get_implementing_countries().await.unwrap();
        self.last_updated = Instant::now();
    }
    // Update Data if needed, when accessed
    async fn get(&mut self) -> &ImplementingCountries {
        if self.is_fresh().await {
            return &self.data;
        } else {
            self.update().await;
            return &self.data;
        }
    }
}

//impl Default for Cache<ImplementingCountries> {
//    fn default() -> Self {
//        Cache {
//            last_updated: Instant::now(),
//            //data: ImplementingCountries { countries: HashSet::new() },
//            //data: tokio::spawn(async { external_data::get_implementing_countries() }).,
//            data: {
//                let rt = Runtime::new().unwrap();
//                let handle = rt.handle();
//                handle.spawn(external_data::get_implementing_countries()).await.unwrap().unwrap()
//            },
//        }
//    }
//}
