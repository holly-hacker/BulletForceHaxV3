mod cli_args;
mod connect_util;
mod utils;

use bulletforce_client::{
    lobby::{LobbyConnectionSettings, LobbyState},
    photon_lib::PhotonObject,
};
use cli_args::CliArgs;
use connect_util::drive_client_loop;
use tracing::{debug, error, info, trace};
use tracing_subscriber::util::SubscriberInitExt;
use utils::generate_uuid_v4;

fn main() {
    let args: CliArgs = argh::from_env();
    init_logging();

    let settings = LobbyConnectionSettings {
        app_version: "1.104.5_HC_1.105".into(),
        user_id: generate_uuid_v4(),
        region: args.region,
    };

    let mut last_game_stats = (0, 0, 0, 0);
    let match_info = drive_client_loop(settings, |client| {
        match client.get_state() {
            LobbyState::ReadyNoLobby { .. } => {
                info!("Connected to server, joining lobby");
                if let Err(e) = client.join_lobby() {
                    error!("Error while trying to join lobby: {e}");
                }
            }
            LobbyState::Ready {
                games, app_stats, ..
            } => {
                // log lobby state
                let new_game_stats = match app_stats {
                    Some(app_stats) => (
                        games.len(),
                        app_stats.master_peer_count,
                        app_stats.peer_count,
                        app_stats.room_count,
                    ),
                    None => (games.len(), 0, 0, 0),
                };

                if new_game_stats != last_game_stats {
                    info!(
                        "Game count: {}, master peers: {}, peers: {}, rooms: {}",
                        new_game_stats.0, new_game_stats.1, new_game_stats.2, new_game_stats.3,
                    );

                    last_game_stats = new_game_stats;
                }

                // look for target player
                let key = games
                    .iter()
                    .find(|(k, v)| {
                        // bullet force
                        if let Some(PhotonObject::String(str)) = v.custom_properties.get("roomName")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched bf name");
                                return true;
                            }
                        }

                        // some other game, version is 1.0.x
                        if let Some(PhotonObject::String(str)) = v.custom_properties.get("RoomName")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched other game 1");
                                return true;
                            }
                        }

                        // platform=mobile, version=newfps-333
                        if let Some(PhotonObject::String(str)) =
                            v.custom_properties.get("matchname")
                        {
                            if str.contains(&args.lobby_name_segment) {
                                debug!(matched = str, "Matched other game 2");
                                return true;
                            }
                        }

                        if k.contains(&args.lobby_name_segment) {
                            debug!(matched = k, "Matched game key");
                            return true;
                        }

                        false
                    })
                    .map(|(k, _v)| k);

                if let Some(key) = key {
                    // found game to join
                    if let Err(e) = client.join_game(key.clone()) {
                        error!("Error while trying to join game: {e}");
                    }
                }
            }
            LobbyState::ReadyToJoinGame {
                token,
                room_name,
                address,
            } => {
                return Some((token.clone(), room_name.clone(), address.clone()));
            }
            _ => (),
        }

        None
    });

    let Some((token, room_name, address)) = match_info else {
        return;
    };

    info!(
        "Found lobby named '{}' in game {room_name}, can join at address {address} with token {token}",
        args.lobby_name_segment
    );
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish()
        .init();

    info!("Info log enabled!");
    debug!("Debug log enabled!");
    trace!("Trace log enabled!");
}
