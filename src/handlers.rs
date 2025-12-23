use axum::{extract::Query, response::Response};
use http::StatusCode;

#[derive(serde::Deserialize)]
pub(super) struct WebfingerQuery {
    resource: Option<String>,
}

pub(super) async fn webfinger(Query(query): Query<WebfingerQuery>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            include_str!("webfinger_response.json").replace(
                "REPLACEME",
                query
                    .resource
                    .as_deref()
                    .unwrap_or("acct:avery@catpowered.net"),
            ),
        ))
        .unwrap()
}
