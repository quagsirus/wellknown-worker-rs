use axum::{Router, routing::get};
use tower_service::Service;
use worker::*;

mod handlers;

fn router() -> Router {
    Router::new()
        .route("/.well-known/openpgpkey/policy", get(|| async { "" }))
        .route(
            "/.well-known/openpgpkey/hu/{localpart}",
            get(handlers::web_key_directory),
        )
        .route(
            "/.well-known/security.txt",
            get(|| async { include_str!("security.txt") }),
        )
        .route("/.well-known/webfinger", get(handlers::webfinger))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}
