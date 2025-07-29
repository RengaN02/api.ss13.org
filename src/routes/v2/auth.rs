use rocket::{get, http::Status, response::Redirect, State};
use crate::{
    config::Config,
    Database,
    database::{error::Error, *},
    http::{
        self,
        discord::{get_oauth_response, get_user_by_token, User},
    }
};
use super::Json;
use tracing::info;
use serde_json::{from_str, Value};
use serde::{Serialize, Deserialize};

#[get("/auth/login?<code>")]
pub async fn discord_login(
    code: &str,
    config: &State<Config>,
) -> Redirect {
    Redirect::found(format!(
        "https://discord.com/oauth2/authorize?client_id={}&response_type=code&redirect_uri={}&scope=identify&state={}",
        &config.discord_oauth.client_id,
        &config.discord_oauth.redirect_url,
        code
    ))
}

#[get("/auth/callback?<code>&<state>")]
pub async fn discord_callback(
    code: &str,
    state: &str,
    database: &State<Database>,
    config: &State<Config>,
) -> Result<Json<User>, Status> {
    if !state.chars().all(char::is_alphanumeric) {
        return Err(Status::InternalServerError);
    }

    let row_id = match check_auth_request(state, &database.pool).await {
        Ok(row_id) => row_id,
        Err(_) => return Err(Status::InternalServerError),
    };

    let oauth_response = match get_oauth_response(code, &config).await {
        Ok(oauth_response) => oauth_response,
        Err(_) => return Err(Status::InternalServerError),
    };

    let user = match get_user_by_token(oauth_response.access_token.as_str()).await {
        Ok(user) => user,
        Err(_) => return Err(Status::InternalServerError),
    };

    let ckey = match get_ckey_by_discord_id(user.id.as_str(), &database.pool).await {
        Ok(ckey) => Some(ckey),
        Err(Error::NotLinked) => None,
        Err(_) => return Err(Status::InternalServerError),
    };

    let id = match approve_auth_request(row_id, "discord", user.id.as_str(), user.username.as_str(), ckey.as_deref(), &database.pool).await {
        Ok(ckey) => ckey,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json::Ok(user))
}

//http://localhost:3000/v2/auth/login?code=1
