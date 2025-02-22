mod features;
mod networking;

use networking::PacketAction;
use tracing::{debug, error, info, trace};
use tracing_subscriber::FmtSubscriber;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
