use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::{IntoParams, IntoResponses, ToSchema};

// get countries
#[derive(Deserialize, Clone)]
pub struct ImplementingCountries {
    pub(crate) countries: HashSet<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct NagoyaCheckData {
    // TODO: Use enum instead of string? E.g. crate iso3166
    // TODO: Use additional validation
    pub(crate) researcher_affils: HashSet<String>,
    //#[validate(length(min = 3, max = 3))]
    pub(crate) probe_country: String,
}

#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
pub struct NagoyaResponse {
    pub(crate) check_result: bool,
    pub(crate) status_code: u16,
}

#[derive(Serialize, IntoResponses, ToSchema)]
#[response(status = 200)]
pub struct GenericResponse {
    pub(crate) message: String,
    pub(crate) status_code: u16,
}
