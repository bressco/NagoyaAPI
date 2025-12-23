use crate::models::{AppState, Config, ImplementingCountries};
use axum::Router;
use axum::routing::{get, post};
use std::collections::HashMap;
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
    dotenvy::dotenv().expect("Failed to read .env file");
    let env_map: HashMap<String, String> = dotenvy::vars().collect();

    // Load List of Countries implementing measures according to the Nagoya Protocol
    // TODO: Use timestamped struct for caching
    let implementing_countries: ImplementingCountries =
        external_data::get_implementing_countries().await.unwrap();

    //let server_address = dotenvy::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_address: &str = env_map
        .get("SERVER_HOST")
        .map(|s| s.as_str())
        .unwrap_or("127.0.0.1");
    let server_port = env_map
        .get("SERVER_PORT")
        .map(|s| s.as_str())
        .unwrap_or("3125")
        .parse::<u16>()
        .expect("Please select a valid port number of between 0 and 65535");

    let config = Config {
        nominatim_host: env_map.get("NOMINATIM_HOST").unwrap().to_string(),
        server_host: server_address.to_string(),
        server_port,
    };

    let state = AppState {
        implementing_countries,
        config: config.clone(),
    };

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await
            .unwrap();

    let app = Router::new()
        .route("/nagoya_check_cc", post(api::nagoya_check_country_code))
        .route("/nagoya_check_geo", post(api::nagoya_check_geocoordinates))
        .route("/openapi.json", get(api::openapi))
        .route("/health", get(api::health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/docs", ApiDoc::openapi()))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {}
