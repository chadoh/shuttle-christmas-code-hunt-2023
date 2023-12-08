use axum::{
    routing::get, Router, http::StatusCode, extract::Path, response::IntoResponse,
};

async fn hello_world() -> &'static str {
    "Hello, world!\n"
}

async fn fake_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "You fucked up, good job\n")
}

/// Accepts two numbers, `num1` & `num2`.
///
/// Returns `(num1 XOR num2) POW 3`
async fn bit_sorter(
    Path((num1, num2)): Path<(i128, i128)>,
) -> impl IntoResponse {
    format!(
        "{}\n",
        (num1 ^ num2).pow(3)
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/:num1/:num2", get(bit_sorter))
        .route("/-1/error", get(fake_error))
        .route("/", get(hello_world));

    Ok(router.into())
}
