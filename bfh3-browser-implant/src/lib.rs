use photon_lib::photon_message::PhotonMessage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"This is the start function".into());
}

#[wasm_bindgen]
pub fn on_ws_open(url: &str) {
    web_sys::console::log_3(
        &"%cWS Hook".into(),
        &"color:orange".into(),
        &format!("Connection opened to {url}").into(),
    );
}

#[wasm_bindgen]
pub async fn on_ws_send(message: Vec<u8>, url: &str) -> Option<Vec<u8>> {
    let res = PhotonMessage::from_websocket_bytes(&mut message.as_slice());

    web_sys::console::log_3(
        &"%cWS Hook".into(),
        &"color:orange".into(),
        &format!("Sending message to {url}: {res:?}").into(),
    );

    Some(message)
}

#[wasm_bindgen]
pub async fn on_ws_recv(message: Vec<u8>, origin: &str) -> Option<Vec<u8>> {
    let res = PhotonMessage::from_websocket_bytes(&mut message.as_slice());

    web_sys::console::log_3(
        &"%cWS Hook".into(),
        &"color:orange".into(),
        &format!("Receiving message from {origin}: {res:?}").into(),
    );

    Some(message)
}

#[wasm_bindgen]
pub fn on_ws_close(url: &str) {
    web_sys::console::log_3(
        &"%cWS Hook".into(),
        &"color:orange".into(),
        &format!("Connection to {url} closed").into(),
    );
}
