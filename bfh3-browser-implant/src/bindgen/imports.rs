use wasm_bindgen::prelude::*;

use crate::features::DevtoolsMessage;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_name = "sendMessageToDevtools")]
    pub fn send_message_to_devtools(msg: DevtoolsMessage);
}
