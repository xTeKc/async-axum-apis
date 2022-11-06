mod route_one;
use route_one::route_one;

#[allow(unused)]
use axum::{body::Body, routing::{get, post}, Router};

pub async fn create_routes() -> Router<Body> {
    Router::new().route("/one", get(route_one))
}
