use wasm_bindgen::prelude::*;

use crate::features::{DevtoolsMessage, GameData, LobbyData};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_name = "sendMessageToDevtools")]
    pub fn send_message_to_devtools(msg: DevtoolsMessage);

    #[wasm_bindgen(js_name = "sendLobbyData")]
    pub fn send_lobby_data(msg: LobbyData);

    #[wasm_bindgen(js_name = "sendGameData")]
    pub fn send_game_data(msg: GameData);
}
