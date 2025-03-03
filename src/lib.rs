use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;
use serde_json;

pub async fn yes() -> impl axum::response::IntoResponse {
    let response = serde_json::json!({
        "code": 200,
        "data": "Yes friends",
        "msg": null
    });
    axum::Json(response)
}

pub async fn root() -> impl axum::response::IntoResponse {
    let response = serde_json::json!({
        "code": 200,
        "data": "Hello 🐷💩",
        "msg": null
    });
    axum::Json(response)
}

fn router() -> Router {
    Router::new().route("/", get(root)).route("/yes", get(yes))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}
