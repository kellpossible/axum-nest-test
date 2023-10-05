use axum::{Router, routing::post};

async fn handler() -> &'static str {
    "Hello World"
}

pub fn router() -> Router {
    Router::new()
        .route("/some.operation", post(handler))
}
