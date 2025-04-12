mod utils;
mod websocket_client;

use bulletforce_client::{LobbyConnectionSettings, LobbyState, Region};
use tracing::{debug, error, info, trace};
use tracing_subscriber::util::SubscriberInitExt;
use utils::generate_uuid_v4;
use websocket_client::connect_lobby;

fn main() {
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

    let settings = LobbyConnectionSettings {
        app_version: "1.104.5_HC_1.105".into(),
        user_id: generate_uuid_v4(),
        region: Region::NorthAmerica,
    };

    let mut last_game_stats = (0, 0, 0, 0);
    connect_lobby(settings, |client| {
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
            }
            _ => (),
        }

        None::<()>
    });
}
