use axum::{routing::get, Router, http::StatusCode};

async fn hello_world() -> &'static str {
    "Hello, world!\n"
}

async fn fake_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "You fucked up, good job\n")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/-1/error", get(fake_error))
        .route("/", get(hello_world));

    Ok(router.into())
}
