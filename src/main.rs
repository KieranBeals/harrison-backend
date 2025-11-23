use std::{env, net::SocketAddr, rc::Rc, sync::Arc};

use axum::{Router, routing::get};
use dotenvy::dotenv;
use tracing::info;

struct APIKey(Box<str>);

struct AppState {
    api_key: APIKey,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Could not initialize tracing!");

    let _ = dotenv(); // No need to error here since all we care about is the actual values
    let api_key = env::var("API_KEY").expect("API_KEY not found in env!");

    let app_state = Arc::new(AppState {
        api_key: APIKey(api_key.into()),
    });

    let app = Router::new()
        .route("/dummy_data", get(dummy_data))
        .with_state(app_state);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect(&format!(
            "Could not bind TCP Listener on address: {}",
            address
        ));

    info!("Listening on {}", address);
    axum::serve(listener, app)
        .await
        .expect("Could not serve app!");
}

async fn dummy_data() -> String {
    let response = serde_json::json!({
        "image": "data:image/png;base64,encoded_string" // TODO: Ask william if I can use his image
    });

    response.to_string()
}
