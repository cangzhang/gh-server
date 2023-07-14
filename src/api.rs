use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::{error, info};

pub const GH_URL: &str = "https://raw.githubusercontent.com";

pub async fn get_file(
    Extension(agent): Extension<ureq::Agent>,
    Path((user, repo, rev, path)): Path<(String, String, String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = format!("{GH_URL}/{user}/{repo}/{rev}/{path}");
    info!("{url}");
    let body: String = agent
        .get(&url)
        .call()
        .map_err(|err| {
            error!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_string()
        .map_err(|err| {
            error!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(body)
}
