use crate::views::DashboardPage;
use crate::views::IndexPage;
use crate::views::SearchResponse;

use askama_axum::IntoResponse;
use axum::Form;
use thiserror::Error;
use tracing::error;
use tracing::info;

use super::Globals;

use std::sync::Arc;

use axum::extract::State;

use axum::extract::Path;

pub(crate) async fn get_dashboard(
    Path(id): Path<String>,
    State(state): State<Arc<Globals>>,
) -> impl IntoResponse {
    let (name, departures) = state.client.get_schedules(&id).await.unwrap();
    DashboardPage {
        station_name: name,
        departures,
    }
}

#[derive(serde::Deserialize)]
pub struct SearchForm {
    search: String,
}

pub async fn post_search(
    State(state): State<Arc<Globals>>,
    Form(data): Form<SearchForm>,
) -> Result<SearchResponse, SearchError> {
    if data.search.is_empty() {
        return Ok(SearchResponse { stations: vec![] });
    }
    let data = state
        .client
        .search_station(&data.search)
        .await
        .map_err(|e| SearchError { source: e })?;
    Ok(SearchResponse { stations: data })
}

#[derive(Error, Debug)]
#[error("Failed to call the API")]
pub struct SearchError {
    source: anyhow::Error,
}

impl IntoResponse for SearchError {
    fn into_response(self) -> axum::response::Response {
        error!(err = ?self);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[tracing::instrument]
pub async fn get_index() -> impl IntoResponse {
    info!("Hello");
    IndexPage {}
}
