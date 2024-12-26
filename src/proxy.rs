use std::convert::Infallible;
use axum::{
    extract::{Request, State},
    response::Response,
    body::{self, Body},
    http::StatusCode,
};
use crate::AppState;

pub async fn proxy_handler(
    State(state): State<AppState>,
    request: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let uri = format!("{}{}", state.api.relay, request.uri());

    // Check for the secret Key
    let secret_header = request.headers().get("X-GARDEN-DEMO-KEY");
    if secret_header.is_none() || 
       secret_header.unwrap().to_str().unwrap_or("") != state.secret_key {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap());
    }

    println!("Forwarding request to : {}", &uri);

    let mut jwt_manager = state.jwt_manager.lock().await;

    let jwt_token = match jwt_manager.get_token(state.signer.clone()).await{
        Ok(token) => token,
        Err(_) => {
            return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap());
        }
    };

    let client = &state.client;

    let mut forwarded_request = client.request(request.method().clone(), &uri);
    forwarded_request = forwarded_request.header("Authorization", format!("Bearer {}", jwt_token));

    let max_size = 1024 * 1024;
    let body_bytes = body::to_bytes(request.into_body(), max_size)
        .await
        .unwrap_or_default();

    forwarded_request = forwarded_request.header("Content-Type", "application/json");
    forwarded_request = forwarded_request.body(body_bytes);

    match forwarded_request.send().await {
        Ok(response) => {
            let status = response.status();
            let body_bytes = response.bytes().await.unwrap_or_default();
            let forwarded_response = Response::builder()
                .status(status)
                .header("Content-Type", "application/json")
                .body(Body::from(body_bytes))
                .unwrap();
            Ok(forwarded_response)
        }
        Err(_) => {
            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .header("Content-Type", "application/json")
                .body(Body::from("Bad Gateway"))
                .unwrap())
        }
    }
}