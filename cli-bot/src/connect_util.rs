use std::str::FromStr as _;

use bulletforce_client::{ClientImpl, ClientWrapper};
use tracing::{debug, error, info, trace, warn};
use tungstenite::{ClientRequestBuilder, Message, http::Uri};

pub fn drive_client_loop<TClient: ClientImpl, TResp, F>(
    settings: TClient::Settings,
    mut callback: F,
) -> Option<TResp>
where
    F: FnMut(&mut ClientWrapper<TClient>) -> Option<TResp>,
{
    let mut client = ClientWrapper::<TClient>::create(settings);

    let uri = match client.get_url() {
        std::borrow::Cow::Borrowed(str) => Uri::from_static(str),
        std::borrow::Cow::Owned(string) => Uri::from_str(&string).expect("should have valid url"),
    };
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
            debug!("Callback in client loop returned value, exiting loop");
            return Some(ret);
        }
    }
}
