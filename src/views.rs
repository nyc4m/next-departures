use askama::Template;

use crate::sncf;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardPage {
    pub station_name: String,
    pub departures: Vec<sncf::Departure>,
}

#[derive(Template)]
#[template(path = "search-result.html")]
pub struct SearchResponse {
    pub stations: Vec<sncf::Station>,
}
