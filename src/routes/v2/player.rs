use rocket::{get, http::Status, State};
use serde_json::{json, Value};

use crate::{
    config::Config,
    database::{error::Error, *},
    Database,
};

use super::{common::ApiKey, Json};

#[get("/player?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Player>, Status> {
    match get_player(ckey, &database.pool).await {
        Ok(player) => Ok(Json::Ok(player)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/ban?<ckey>&<permanent>&<since>")]
pub async fn ban(
    ckey: &str,
    permanent: Option<bool>,
    since: Option<&str>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<Ban>>, Status> {
    match get_ban(ckey, permanent.unwrap_or(false), since, &database.pool).await {
        Ok(bans) => Ok(Json::Ok(bans)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/characters?<ckey>")]
pub async fn characters(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<(String, i64)>>, Status> {
    match get_characters(ckey, &database.pool).await {
        Ok(characters) => Ok(Json::Ok(characters)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/roletime?<ckey>")]
pub async fn roletime(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<PlayerRoletime>>, Status> {
    match get_roletime(ckey, &database.pool).await {
        Ok(roletimes) => Ok(Json::Ok(roletimes)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/roletime/top?<job>")]
pub async fn top(
    job: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<JobRoletime>>, Status> {
    let Ok(roletimes) = get_top_roletime(job, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(Json::Ok(roletimes))
}

#[get("/player/activity?<ckey>")]
pub async fn activity(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<(String, i64)>>, Status> {
    match get_activity(ckey, &database.pool).await {
        Ok(activity) => Ok(Json::Ok(activity)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/discord?<ckey>&<discord_id>")]
pub async fn discord(
    ckey: Option<&str>,
    discord_id: Option<&str>,
    database: &State<Database>,
    config: &State<Config>,
    _api_key: ApiKey,
) -> Result<Json<Value>, Status> {
    if ckey.is_some() ^ discord_id.is_none() {
        return Err(Status::BadRequest);
    }

    if let Some(ckey) = ckey {
        return match fetch_discord_by_ckey(ckey, &config.discord.token, &database.pool).await {
            Ok(user) => Ok(Json::Ok(json!(user))),
            Err(Error::PlayerNotFound) => Err(Status::NotFound),
            Err(Error::NotLinked) => Err(Status::Conflict),
            Err(_) => Err(Status::InternalServerError),
        };
    } else if let Some(discord_id) = discord_id {
        return match get_ckey_by_discord_id(discord_id, &database.pool).await {
            Ok(ckey) => Ok(Json::Ok(Value::String(ckey))),
            Err(Error::NotLinked) => Err(Status::Conflict),
            Err(_) => Err(Status::InternalServerError),
        };
    }

    unreachable!()
}

#[get("/player/achievements?<ckey>")]
pub async fn achievements(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<serde_json::Value>, Status> {
    match get_achievements(ckey, &database.pool).await {
        Ok(achievements) => Ok(Json::Ok(achievements)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
