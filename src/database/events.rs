use chrono::NaiveDateTime;
use rocket::{
    futures::StreamExt as _
};
use serde::Serialize;
use sqlx::{Executor as _, MySqlPool, Row as _};
use serde_json::Value;
use std::collections::HashMap;
use super::error::Error;

use crate::{
    byond::{self, ServerStatus},
    config::{Config, Server},
};

#[derive(Debug, Serialize)]
pub struct Feedback {
    pub round_id: Option<u32>,
    pub key_name: String,
    pub key_type: String,
    pub json: Value,
    #[serde(with = "crate::serde::datetime")]
    pub datetime: NaiveDateTime,
}

pub async fn get_feedback_list(
    key_name: &str,
    key_type: &str,
    limit: Option<i32>,
    pool: &MySqlPool,
) -> Result<Vec<Feedback>, Error> {
    let limit = limit.unwrap_or(100);

    let current_round_id = get_round_id().await?;
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT datetime, round_id, key_name, key_type, json FROM feedback WHERE key_name = ? AND key_type = ?".to_string();

    if current_round_id.is_some() {
        sql.push_str(" AND round_id < ? AND round_id >= ?");
    }
    sql.push_str(" ORDER BY datetime DESC LIMIT ?");

    let mut query = sqlx::query(&sql).bind(key_name).bind(key_type);

    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id).bind(current_round_id as i32 - limit);
    }

    query = query.bind(limit);

    let mut feedbacks = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let feedback = row?;

            let feedback = Feedback {
                datetime: feedback.try_get("datetime")?,
                round_id: feedback.try_get("round_id")?,
                key_name: feedback.try_get("key_name")?,
                key_type: feedback.try_get("key_type")?,
                json: feedback.try_get("json")?,
            };

            feedbacks.push(feedback);
        }
    }

    connection.close().await?;

    Ok(feedbacks)
}

#[derive(Debug, Serialize)]
pub struct Death {
    pub name: String,
    pub job: String,
    pub pod: String,
    pub bruteloss: u16,
    pub fireloss: u16,
    pub oxyloss: u16,
    pub toxloss: u16,
    pub last_words: Option<String>,
    pub suicide: bool,
    pub round_id: Option<u32>,
    #[serde(with = "crate::serde::datetime")]
    pub tod: NaiveDateTime,
}

pub async fn get_deaths(fetch_size: Option<u32>, page: Option<u32>, pool: &MySqlPool) -> Result<Vec<Death>, Error> {
    let current_round_id = get_round_id().await?;
    let fetch_size = fetch_size.unwrap_or(20);
    let page = page.unwrap_or(1);

    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT name, job, pod, bruteloss, fireloss, oxyloss, toxloss, last_words, suicide, round_id, tod FROM death".to_string();

    if current_round_id.is_some() {
        sql.push_str(" WHERE round_id < ?");
    }

    sql.push_str(" ORDER BY round_id DESC LIMIT ? OFFSET ?");

    let mut query = sqlx::query(&sql);

    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id);
    }
    query = query.bind(fetch_size).bind(page);

    let mut deaths = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let death = row?;

            let death = Death {
                name: death.try_get("name")?,
                job: death.try_get("job")?,
                pod: death.try_get("pod")?,
                bruteloss: death.try_get("bruteloss")?,
                fireloss: death.try_get("fireloss")?,
                oxyloss: death.try_get("oxyloss")?,
                toxloss: death.try_get("toxloss")?,
                last_words: death.try_get("last_words")?,
                suicide: death.try_get("suicide")?,
                round_id: death.try_get("round_id")?,
                tod: death.try_get("tod")?,
            };

            deaths.push(death);
        }
    }

    connection.close().await?;

    Ok(deaths)
}

#[derive(Debug, Serialize)]
pub struct Citation {
    pub sender: String,
    pub recipient: String,
    pub crime: String,
    pub fine: Option<i32>,
    pub round_id: Option<u32>,
    #[serde(with = "crate::serde::datetime")]
    pub timestamp: NaiveDateTime,
}

pub async fn get_citations(fetch_size: Option<u32>, page: Option<u32>, pool: &MySqlPool) -> Result<Vec<Citation>, Error> {
    let current_round_id = get_round_id().await?;
    let fetch_size = fetch_size.unwrap_or(20);
    let page = page.unwrap_or(1);

    let mut connection = pool.acquire().await?;

    let mut sql =
        "SELECT round_id, sender_ic, recipient, crime, fine, timestamp FROM citation".to_string();

    if current_round_id.is_some() {
        sql.push_str(" WHERE round_id < ?");
    }

    sql.push_str(" ORDER BY round_id DESC LIMIT ? OFFSET ?");

    let mut query = sqlx::query(&sql);

    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id);
    }

    query = query.bind(fetch_size).bind(page);

    let mut citations = Vec::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let citation = row?;

            let citation = Citation {
                round_id: citation.try_get("round_id")?,
                sender: citation.try_get("sender_ic")?,
                recipient: citation.try_get("recipient")?,
                crime: citation.try_get("crime")?,
                fine: citation.try_get("fine")?,
                timestamp: citation.try_get("timestamp")?,
            };

            citations.push(citation);
        }
    }

    connection.close().await?;

    Ok(citations)
}

pub async fn get_death_counts(limit: Option<i32>, pool: &MySqlPool) -> Result<HashMap<Option<u32>, i32>, Error> {
    let limit = limit.unwrap_or(100);
    let current_round_id = get_round_id().await?;
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT round_id, COUNT(*) as death_count FROM death GROUP BY round_id".to_string();

    if current_round_id.is_some() {
        sql.push_str(" WHERE round_id < ? AND round_id >= ?");
    }
    sql.push_str(" ORDER BY round_id DESC LIMIT ?");

    let mut query = sqlx::query(&sql);

    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id).bind(current_round_id as i32 - limit);
    }
    query = query.bind(limit);

    let mut death_counts = HashMap::new();
    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let death = row?;
            let round_id = death.try_get("round_id")?;
            let death_count = death.try_get("death_count")?;
            if let (Some(round_id), Some(death_count)) = (round_id, death_count) {
                death_counts.insert(round_id, death_count);
            }
        }
    }

    Ok(death_counts)
}

pub async fn get_citation_counts(limit: Option<i32>, pool: &MySqlPool) -> Result<HashMap<Option<u32>, i32>, Error> {
    let limit = limit.unwrap_or(100);
    let current_round_id = get_round_id().await?;
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT round_id, COUNT(*) as citation_count FROM citation GROUP BY round_id".to_string();
    if current_round_id.is_some() {
        sql.push_str(" WHERE round_id < ? AND round_id >= ?");
    }
    sql.push_str(" ORDER BY round_id DESC LIMIT ?");

    let mut query = sqlx::query(&sql);

    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id).bind(current_round_id as i32 - limit);
    }
    query = query.bind(limit);

    let mut citation_counts = HashMap::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let citation = row?;
            let round_id = citation.try_get("round_id")?;
            let citation_count = citation.try_get("citation_count")?;
            if let (Some(round_id), Some(citation_count)) = (round_id, citation_count) {
                citation_counts.insert(round_id, citation_count);
            }
        }
    }

    Ok(citation_counts)
}

pub async fn get_round_durations(limit: Option<i32>, pool: &MySqlPool) -> Result<HashMap<i32, i64>, Error> {
    let limit = limit.unwrap_or(100);
    let current_round_id = get_round_id().await?;
    let mut connection = pool.acquire().await?;

    let mut sql = "SELECT id, start_datetime, end_datetime FROM round".to_string();

    if current_round_id.is_some() {
        sql.push_str(" WHERE id < ? AND id >= ?");
    }

    sql.push_str(" ORDER BY round_id DESC LIMIT ?");

    let mut query = sqlx::query(&sql);
    if let Some(current_round_id) = current_round_id {
        query = query.bind(current_round_id).bind(current_round_id as i32 - limit);
    }

    query = query.bind(limit);

    let mut durations = HashMap::new();

    {
        let mut rows = connection.fetch(query);

        while let Some(row) = rows.next().await {
            let duration = row?;
            let round_id = duration.try_get("id")?;
            let start_datetime: Option<NaiveDateTime> = duration.try_get("start_datetime")?;
            let end_datetime: Option<NaiveDateTime> = duration.try_get("end_datetime")?;

            if let (Some(round_id), Some(start), Some(end)) = (round_id, start_datetime, end_datetime) {
                let duration_seconds = end.signed_duration_since(start).num_seconds();
                let duration_minutes = (duration_seconds / 60) as i64;
                durations.insert(round_id, duration_minutes);
            }
        }
    }

    Ok(durations)
}

pub async fn get_player_counts(limit: Option<i32>, pool: &MySqlPool) -> Result<HashMap<u32, i32>, Error> {
    let limit = limit.unwrap_or(100);
    match get_feedback_list("round_end_stats", "nested tally", Some(limit), pool).await {
        Ok(feedbacks) => {
            let mut player_counts = HashMap::new();
            for feedback in &feedbacks {
                let player_count = feedback.json["data"]["players"]["total"]
                    .as_i64()
                    .map(|n| n as i32)
                    .unwrap_or(0);

                if let Some(round_id) = feedback.round_id {
                    player_counts.insert(round_id, player_count);
                }
            }
            Ok(player_counts)
        }
        Err(_) => Ok(HashMap::new()),
    }
}

pub async fn get_round_id() -> Result<Option<u32>, Error> {
    let config = Config::read_from_file().unwrap(); 
    let server: &Server = config.servers.iter().next().ok_or(Error::GameServerError)?;
    let status: Option<ServerStatus> = byond::status(&server.address).await.ok();

    if let Some(status) = status {
        let round_id = status.round_id;
        Ok(Some(round_id))
    } else {
        Ok(None)
    }
}
