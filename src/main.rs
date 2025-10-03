use axum::Router;
use axum::routing::{get, post};
use dotenvy::dotenv;
use utoipa::OpenApi;

mod handlers;
mod helpers;
mod models;
mod nagoya_check;

#[derive(OpenApi)]
#[openapi(paths(
    handlers::openapi,
    handlers::nagoya_check_wrapper,
    handlers::health_check
))]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load env
    dotenv().ok();

    // Load List of Countries implementing measures according to the Nagoya Protocol
    let implementing_countries = helpers::get_implementing_countries().unwrap();

    let server_address = dotenvy::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let service_port = dotenvy::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Please select a valid port number of between 0 and 65535");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_address, service_port))
        .await
        .unwrap();

    let app = Router::new()
        .route("/nagoya_check", post(handlers::nagoya_check_wrapper))
        .route("/openapi.json", get(handlers::openapi))
        .route("/health", get(handlers::health_check))
        //.merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .with_state(implementing_countries);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::models::ImplementingCountries;
    use crate::nagoya_check::{are_affils_from_probe_country, is_probe_in_implementing_country};
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_are_affils_from_probe_country() {
        let data_included = HashSet::from_iter(vec!["AUS".to_string(), "DEU".to_string()]);
        let data_not_included = HashSet::from_iter(vec!["DEU".to_string()]);
        let data_empty = HashSet::from_iter(vec!["".to_string()]);
        let probe: &str = "AUS";

        assert_eq!(
            are_affils_from_probe_country(&data_included, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            are_affils_from_probe_country(&data_not_included, &probe)
                .await
                .unwrap(),
            false
        );
        assert_eq!(
            are_affils_from_probe_country(&data_empty, &probe)
                .await
                .unwrap(),
            false
        );
    }

    #[tokio::test]
    async fn test_probe_in_implementing_country() {
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

        assert_eq!(
            is_probe_in_implementing_country(&data_included, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_included_single, &probe)
                .await
                .unwrap(),
            true
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_not_included, &probe)
                .await
                .unwrap(),
            false
        );
        assert_eq!(
            is_probe_in_implementing_country(&data_empty, &probe)
                .await
                .unwrap(),
            false
        );
    }
}
