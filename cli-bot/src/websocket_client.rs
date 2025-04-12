use bulletforce_client::{BulletForceLobbyClient, LobbyConnectionSettings};
use tracing::{debug, error, info, trace, warn};
use tungstenite::{ClientRequestBuilder, Message, http::Uri};

pub fn connect_lobby<T, F>(settings: LobbyConnectionSettings, mut callback: F) -> Option<T>
where
    F: FnMut(&mut BulletForceLobbyClient) -> Option<T>,
{
    let mut client = BulletForceLobbyClient::create(settings);

    let uri = Uri::from_static(client.get_lobby_url());
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

        // run client logic
        if let Some(ret) = callback(&mut client) {
            debug!("Callback for lobby loop returned value, exiting loop");
            return Some(ret);
        }
    }
}
