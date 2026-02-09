// SPDX-FileCopyrightText: 2025 Constantin Breß <constantin.bress@partner.kit.edu>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

use crate::models::{AppState, Config, ImplementingCountries};
use axum::Router;
use axum::routing::{get, post};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod external_data;
mod models;
mod nagoya_check;

#[derive(OpenApi)]
#[openapi(paths(
    api::openapi,
    api::nagoya_check_country_code,
    api::nagoya_check_geocoordinates,
    api::health_check
))]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load env
    dotenvy::dotenv().expect("No .env file found, using defaults");

    // Load List of Countries implementing measures according to the Nagoya Protocol
    // Without the data the service cannot work, thus the panic is justified if the data
    // cannot be fetched
    // TODO: Use timestamped struct for caching
    let implementing_countries: ImplementingCountries =
        external_data::get_implementing_countries().await.unwrap();

    let server_address = dotenvy::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let server_port = dotenvy::var("SERVER_PORT")
        .unwrap_or("3125".to_string())
        .parse::<u16>()
        .expect("Please select a valid port number of between 0 and 65535");

    let config = Config {
        nominatim_host: dotenvy::var("NOMINATIM_HOST")
            // A custom host should be provided to not hog the service provided by OSM
            .expect("Please provide a Nominatim Host")
            .to_string(),
        server_host: server_address.to_string(),
        server_port,
    };

    let state = AppState::new(
        config.clone(),
        implementing_countries,
        Duration::new(
            dotenvy::var("CACHE_TTL").unwrap().parse::<u64>().unwrap(),
            0,
        ),
    );

    let listener = tokio::net::TcpListener::bind(format!(
        "{host}:{port}",
        host = config.server_host,
        port = config.server_port
    ))
    .await
    .unwrap();

    let app = Router::new()
        .route("/nagoya_check_cc", post(api::nagoya_check_country_code))
        .route("/nagoya_check_geo", post(api::nagoya_check_geocoordinates))
        .route("/openapi.json", get(api::openapi))
        .route("/health", get(api::health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/docs", ApiDoc::openapi()))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {}
