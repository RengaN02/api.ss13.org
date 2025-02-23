use rocket::{routes, Build, Rocket};

mod autocomplete;
mod common;
mod discord;
mod events;
mod patreon;
mod player;
mod server;
mod verify;

pub use common::*;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/v2",
        routes![
            patreon::index,
            patreon::patrons,
            player::achievements,
            player::index,
            player::ban,
            player::characters,
            player::roletime,
            player::activity,
            player::top,
            player::discord,
            server::index,
            verify::index,
            verify::unverify,
            discord::user,
            discord::member,
            autocomplete::job,
            autocomplete::ckey,
            autocomplete::ic_name,
            events::deaths,
            events::chart_data,
            events::citations
        ],
    )
}
