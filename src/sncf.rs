use anyhow::Context;
use iso8601::{datetime, DateTime};
use serde_json::Value;

#[derive(Clone)]
pub struct Client {
    base_url: String,
}

#[derive(serde::Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Departure {
    pub direction: String,
    pub time: DateTime,
}

impl Departure {
    pub fn formatted_time(&self) -> String {
        format!("{:0>2}h{:0>2}", self.time.time.hour, self.time.time.minute)
    }
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            base_url: format!("https://{}@api.navitia.io/v1", token),
        }
    }

    pub async fn search_station(&self, name: &str) -> anyhow::Result<Vec<Station>> {
        let url = format!("{}/coverage/sncf/pt_objects?q={}", self.base_url, name);

        let payload: Value = reqwest::get(&url).await.unwrap().json().await.unwrap();
        let stations = payload["pt_objects"]
            .as_array()
            .with_context(|| "pt_objects is not accessible")?
            .iter()
            .filter(|station| station["embedded_type"] == "stop_area")
            .map(|station| Station {
                id: station["id"].as_str().unwrap().to_string(),
                name: station["name"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<_>>();
        Ok(stations)
    }

    pub async fn get_schedules(
        &self,
        station_id: &str,
    ) -> Result<(String, Vec<Departure>), reqwest::Error> {
        let url = format!(
            "{}/coverage/sncf/stop_areas/{}/departures",
            self.base_url, station_id
        );
        let payload: Value = reqwest::get(&url).await.unwrap().json().await.unwrap();
        let name_response = reqwest::get(format!(
            "{}/coverage/sncf/stop_areas/{}",
            self.base_url, station_id,
        ))
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
        let name = name_response["stop_areas"].as_array().unwrap()[0]["label"]
            .as_str()
            .unwrap()
            .to_string();

        let departures: Vec<_> = payload["departures"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| Departure {
                direction: i["display_informations"]["direction"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                time: datetime(i["stop_date_time"]["departure_date_time"].as_str().unwrap())
                    .unwrap(),
            })
            .collect();
        Ok((name, departures))
    }
}
