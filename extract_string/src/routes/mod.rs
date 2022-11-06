mod body_string;
mod route_one;
use body_string::body_string;
use route_one::route_one;

#[allow(unused)]
use axum::{body::Body, routing::{get, post}, Router};

pub async fn create_routes() -> Router<Body> {
    Router::new()
        .route("/one", get(route_one))
        .route("/body_string", post(body_string))
}
