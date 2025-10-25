use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::{IntoParams, IntoResponses, ToSchema};

#[derive(Deserialize, PartialEq, Hash, Eq, Debug)]
pub struct Treaty {
    #[serde(rename = "party")]
    pub(crate) party_date: Option<String>,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Debug)]
pub struct Treaties {
    //#[serde(rename = "XXVII8b", flatten)]
    #[serde(rename = "XXVII8b")]
    pub(crate) nagoya: Treaty,
}

#[derive(Deserialize, PartialEq, Hash, Eq, Debug)]
pub struct NagoyaCountryInfo {
    pub(crate) code2: String,
    pub(crate) code3: String,
    //pub(crate) nagoya_info: NagoyaTreatyInfo,
    //#[serde(flatten)]
    pub(crate) treaties: Treaties,
}

#[derive(Deserialize, Clone)]
pub struct ImplementingCountries {
    pub(crate) countries: HashSet<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct NagoyaCheckData {
    // TODO: Use additional validation
    // TODO: Add data for registered collection
    // TODO: Add data for Certificates
    pub(crate) probe_country: String,
}

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
