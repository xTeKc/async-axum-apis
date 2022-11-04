use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build app with a hello route
    let app = Router::new().route("/hello", get(hello));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> String {
    "Hello, Hello!".to_owned()
}