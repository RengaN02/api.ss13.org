use rocket::{get, http::Status, State};

use crate::{database::*, Database};

use super::{common::ApiKey, Json};
use serde_json::json;

#[get("/events/chart-data?<limit>")]
pub async fn chart_data(
    limit: Option<i32>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<serde_json::Value>>, Status> {
    match get_feedback_list("dynamic_threat", "associative", limit, &database.pool).await {
        Ok(feedbacks) => {
            let death_counts = get_death_counts(limit, &database.pool).await.unwrap_or_default();
            let citation_counts = get_citation_counts(limit, &database.pool).await.unwrap_or_default();
            let round_durations = get_round_durations(limit, &database.pool).await.unwrap_or_default();
            let total_player_counts = get_player_counts(limit, &database.pool).await.unwrap_or_default();

            let processed_data: Vec<serde_json::Value> = feedbacks
                .into_iter()
                .filter_map(|feedback| {
                    let round_id = feedback.round_id;
                    let round_id_i32 = round_id.map(|id| id as i32).unwrap_or(0);
                    let threat_level = feedback.json["data"]["1"]["threat_level"].as_str()?.parse::<i32>().ok()?;
                    let readyed_players = feedback.json["data"]["1"]["player_count"].as_str()?.parse::<i32>().ok()?;
                    let deaths = *death_counts.get(&round_id).unwrap_or(&0);
                    let citations = *citation_counts.get(&round_id).unwrap_or(&0);
                    let round_duration = *round_durations.get(&round_id_i32).unwrap_or(&0);
                    let player_counts = *total_player_counts.get(&round_id.unwrap_or(0)).unwrap_or(&0);

                    Some(json!({
                        "round_id": round_id,
                        "threat_level": threat_level,
                        "total_players": player_counts,
                        "readyed_players": readyed_players,
                        "deaths": deaths,
                        "citations": citations,
                        "round_duration": round_duration
                    }))
                })
                .collect();

            Ok(Json::Ok(processed_data))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/events/deaths?<fetch_size>&<page>")]
pub async fn deaths(
    fetch_size: Option<u32>,
    page: Option<u32>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<Death>>, Status> {
    match get_deaths(fetch_size, page, &database.pool).await {
        Ok(deaths) => Ok(Json::Ok(deaths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/events/citations?<fetch_size>&<page>")]
pub async fn citations(
    fetch_size: Option<u32>,
    page: Option<u32>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<Citation>>, Status> {
    match get_citations(fetch_size, page, &database.pool).await {
        Ok(citations) => Ok(Json::Ok(citations)),
        Err(_) => Err(Status::InternalServerError),
    }
}
