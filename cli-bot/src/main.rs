mod utils;

use bulletforce_client::{BulletForceLobbyClient, LobbyConnectionSettings, LobbyState};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::util::SubscriberInitExt;
use tungstenite::{ClientRequestBuilder, Message, http::Uri};
use utils::generate_uuid_v4;

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
    };
    let mut client = BulletForceLobbyClient::create(settings);

    let uri = Uri::from_static(BulletForceLobbyClient::get_lobby_server());
    let builder =
        ClientRequestBuilder::new(uri).with_header("Sec-WebSocket-Protocol", "GpBinaryV16");

    let (mut ws_stream, _) = tungstenite::connect(builder).expect("failed to connect");
    info!("Connected to server");

    loop {
        // send out queued messages
        let to_send = client.take_messages_to_send();
        if !to_send.is_empty() {
            trace!(amount = to_send.len(), "Sending messages");
            for item in to_send {
                ws_stream.write(item.into()).expect("write item to stream");
            }
            ws_stream.flush().expect("flush stream");
        }

        // feed in incoming messages
        let incoming_message = ws_stream.read().expect("get incoming message");
        if let Message::Binary(bytes) = incoming_message {
            if let Err(e) = client.handle_input(&bytes) {
                error!("Error while handling incoming message: {e}");
            }
        } else {
            warn!(
                "Received message that was not binary: {:?}",
                incoming_message
            );
        }

        // run app-specific logic
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
                if let Some(app_stats) = app_stats {
                    info!(
                        "Game count: {}, master peers: {}, peers: {}, rooms: {}",
                        games.len(),
                        app_stats.master_peer_count,
                        app_stats.peer_count,
                        app_stats.room_count
                    )
                } else {
                    info!("Game count: {}", games.len());
                }
            }
            _ => (),
        }
    }
}
