mod features;
mod networking;

use networking::{PacketAction, PacketDirection, SocketType};
use tracing::{debug, error, info, trace};
use tracing_subscriber::FmtSubscriber;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_name = "sendMessageToDevtools")]
    fn send_message_to_devtools(msg: DevtoolsMessage);
}

#[wasm_bindgen(start)]
fn start() {
    // install panic handler
    console_error_panic_hook::set_once();

    // logging layer
    let fmt_layer = FmtSubscriber::builder()
        .with_ansi(true)
        .without_time()
        .with_writer(tracing_web::MakeWebConsoleWriter::new().with_pretty_level())
        .with_level(false)
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(fmt_layer).expect("set logger");

    info!("Info logging enabled");
    debug!("Debug logging is enabled");
    trace!("Trace logging is enabled");
}

#[wasm_bindgen]
pub fn on_ws_open(url: &str) {
    info!(url, "Connection opened");
}

#[wasm_bindgen]
pub async fn on_ws_send(message: Vec<u8>, url: &str) -> Option<Vec<u8>> {
    trace!(url, "Message sent: {message:?}");
    let res = match networking::handle_packet_send(&message, url) {
        Ok(a) => a,
        Err(err) => {
            error!(url, "Error handling sent ws packet: {err:?}");
            PacketAction::Ignore
        }
    };

    match res {
        PacketAction::Ignore => Some(message),
        PacketAction::Drop => None,
        PacketAction::Modify(new_message) => Some(new_message),
    }
}

#[wasm_bindgen]
pub async fn on_ws_recv(message: Vec<u8>, origin: &str) -> Option<Vec<u8>> {
    trace!(origin, "Message received: {message:?}");
    let res = match networking::handle_packet_recv(&message, origin) {
        Ok(a) => a,
        Err(err) => {
            error!(origin, "Error handling received ws packet: {err:?}");
            PacketAction::Ignore
        }
    };

    match res {
        PacketAction::Ignore => Some(message),
        PacketAction::Drop => None,
        PacketAction::Modify(new_message) => Some(new_message),
    }
}

#[wasm_bindgen]
pub fn on_ws_close(url: &str) {
    info!(url, "Connection closed");
}

#[wasm_bindgen]
pub struct DevtoolsMessage {
    /// The direction the packet was going
    pub direction: PacketDirection,
    /// Which server the socket is connected to
    pub socket_type: SocketType,
    /// The type of the message
    pub message_type: u8,
    /// The raw message, encoded as MessagePack
    #[wasm_bindgen(getter_with_clone)]
    pub message: Vec<u8>,
    /// The high-level message (if any), encoded as MessagePack
    #[wasm_bindgen(getter_with_clone)]
    pub parsed_message: Option<Vec<u8>>,
    /// The parsing error, if any occurred
    #[wasm_bindgen(getter_with_clone)]
    pub error: Option<String>,
}
