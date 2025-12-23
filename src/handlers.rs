use axum::{
    extract::{Path, Query},
    response::Response,
};
use http::StatusCode;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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

pub(super) async fn web_key_directory(
    Path(localpart): Path<String>,
) -> Result<Vec<u8>, StatusCode> {
    let key = WEB_KEY_DIRECTORY_MAP.get(&localpart);
    if key.is_some() {
        return Ok(key.unwrap().to_vec());
    }

    return Err(StatusCode::NOT_FOUND);
}
