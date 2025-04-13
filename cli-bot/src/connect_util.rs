use std::str::FromStr as _;

use bulletforce_client::{ClientImpl, Client};
use tracing::{debug, error, info, trace, warn};
use tungstenite::{ClientRequestBuilder, Message, http::Uri};

pub fn drive_client_loop<TClient: ClientImpl, TResp, F>(
    settings: TClient::Settings,
    mut callback: F,
) -> Option<TResp>
where
    F: FnMut(&mut Client<TClient>) -> Option<TResp>,
{
    let mut client = Client::<TClient>::create(settings);

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
        let incoming_message = match ws_stream.read() {
            Ok(msg) => msg,
            Err(tungstenite::Error::ConnectionClosed) => {
                info!("WebSocket connection was closed by remote");
                break;
            }
            Err(e) => {
                panic!("Unexpected error in ws stream: {e}");
            }
        };
        match incoming_message {
            Message::Binary(bytes) => {
                if let Err(e) = client.handle_input(&bytes) {
                    error!("Error while handling incoming message: {e}");
                }
            }
            Message::Close(frame) => {
                info!("WebSocket connection received close frame: {frame:?}");
                break;
            }
            _ => {
                warn!(
                    "Received message that was not binary: {:?}",
                    incoming_message
                );
            }
        }

        // run client logic
        if let Some(ret) = callback(&mut client) {
            debug!("Callback in client loop returned value, exiting loop");
            return Some(ret);
        }
    }

    None
}
