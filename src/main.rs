mod jwt_manager;
mod signer;
mod relay;
mod proxy;

use axum::{
    routing::{any, get},
    Router
};
use jwt_manager::JwtManager;
use proxy::proxy_handler;
use reqwest::Client;
use signer::LocalSigner;
use tokio::sync::Mutex;
use std::{env, sync::Arc};


#[derive(Clone)]
struct GardenApi{
    relay : String
}

#[derive(Clone)]
struct AppState {
    jwt_manager : Arc<Mutex<jwt_manager::JwtManager>>,
    signer: Arc<Mutex<LocalSigner>>,
    client : Client,
    api : GardenApi,
    secret_key : String
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let signer = Arc::new(Mutex::new(LocalSigner::init()));

    let jwt_manager = Arc::new(Mutex::new(JwtManager::new()));

    let secret_key = env::var("SECRET_KEY").expect("SECRET KEY IS REQUIRED");

    let state = AppState {
        jwt_manager : jwt_manager,
        signer,
        client : Client::new(),
        api : GardenApi{
            relay : String::from("https://evm-swapper-relay.onrender.com")
        },
        secret_key
    };    

    let app = Router::new()
        .route("/", get(home))
        .route("/*path", any(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Garden API Proxy"
}
