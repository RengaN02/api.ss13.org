use std::collections::HashSet;

use chrono::NaiveDateTime;
use rocket::futures::StreamExt as _;
use serde::Serialize;
use sqlx::{Executor as _, MySqlPool, Row as _};
use tracing::info;
use super::error::Error;

pub async fn check_auth_request(
    access_code: &str,
    pool: &MySqlPool,
) -> Result<i32, Error> {
    let mut connection = pool.acquire().await?;

    let row = sqlx::query(
        "SELECT id FROM authentication_requests WHERE access_code = ? AND request_status = 0 AND timestamp >= Now() - INTERVAL 2 hour ORDER BY timestamp DESC LIMIT 1"
    ).bind(access_code) 
    .fetch_one(&mut *connection)
    .await?;

    let id: i32 = row.try_get("id")?;

    connection.close().await?;

    Ok(id)
}

pub async fn approve_auth_request(
    id: i32,
    authentication_method: &str,
    external_uid: &str,
    external_username: &str,
    internal_byond_id: Option<&str>,
    pool: &MySqlPool,
) -> Result<i32, Error> {
    let mut connection = pool.acquire().await?;
    let mut sql =
        "UPDATE authentication_requests SET authentication_method = ?, external_uid = ?, external_username = ?, request_status = 1, timestamp = Now()".to_string();

    if internal_byond_id.is_some() {
        sql.push_str(", internal_byond_id = ?");
    }

    sql.push_str(" WHERE id = ?");

    let mut query = sqlx::query(&sql).bind(authentication_method).bind(external_uid).bind(external_username);

    if let Some(byond_ckey) = internal_byond_id {
        query = query.bind(byond_ckey);
    }

    query = query.bind(id);

    connection.execute(query).await?;
    connection.close().await?;
    Ok(id)
}
