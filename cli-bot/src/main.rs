mod utils;

use bulletforce_client::{BulletForceLobbyClient, LobbyConnectionSettings, LobbyState};
use tracing::{debug, info, trace, warn};
use tracing_subscriber::util::SubscriberInitExt;
use tungstenite::{ClientRequestBuilder, Message, http::Uri};
use utils::generate_uuid_v4;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
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
            client.handle_input(&bytes);
        } else {
            warn!(
                "Received message that was not binary: {:?}",
                incoming_message
            );
        }

        // run app-specific logic
        if let LobbyState::ReadyNoLobby { .. } = client.get_state() {
            client.join_lobby();
        }
    }
}
