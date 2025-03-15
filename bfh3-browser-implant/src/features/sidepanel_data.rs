use std::sync::Mutex;

use crate::networking::{PacketAction, SocketType};

use super::Feature;

use photon_lib::pun::lifting::RoomInfoList;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct LobbyData {
    #[wasm_bindgen(getter_with_clone)]
    pub matches: Vec<LobbyMatch>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct LobbyMatch {
    pub key: String,
    pub players: u8,
    pub max_players: u8,
    pub name: String,
    pub map: String,
    pub platform: String,
    pub version: String,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct GameData {}

enum EitherData {
    Lobby(LobbyData),
    Game(GameData),
}

static DATA: Mutex<Option<EitherData>> = Mutex::new(None);

pub struct SidepanelData;

impl Feature for SidepanelData {
    fn get_name(&self) -> &'static str {
        "sidepanel data"
    }

    fn on_tick(&self) -> anyhow::Result<()> {
        let current_data = DATA.lock().unwrap();

        match current_data.as_ref() {
            Some(EitherData::Lobby(lobby)) => {
                crate::bindgen::send_lobby_data(lobby.clone());
            }
            Some(EitherData::Game(game)) => {
                crate::bindgen::send_game_data(game.clone());
            }
            None => (),
        }

        Ok(())
    }

    fn on_socket_open(&self, socket_type: SocketType) -> anyhow::Result<()> {
        match socket_type {
            SocketType::Lobby => {
                *DATA.lock().unwrap() = Some(EitherData::Lobby(LobbyData::default()));
            }
            SocketType::Game => {
                *DATA.lock().unwrap() = Some(EitherData::Game(GameData::default()));
            }
        }

        Ok(())
    }

    fn on_socket_close(&self, socket_type: SocketType) -> anyhow::Result<()> {
        let current_data = DATA.lock().unwrap();

        match (current_data.as_ref(), socket_type) {
            (Some(EitherData::Lobby(_)), SocketType::Lobby)
            | (Some(EitherData::Game(_)), SocketType::Game) => {
                *DATA.lock().unwrap() = None;
            }
            (Some(_), _) => {
                // we already have data, but it's a different type
                // this can happen if the new socket is opened before the old one is closed, so this is fine
            }
            (None, _) => {
                // socket is already closed, could happen if 2 sockets are closed at once
                // should never happen
            }
        }

        Ok(())
    }

    fn on_gamelist_update(
        &self,
        rooms: &RoomInfoList,
    ) -> anyhow::Result<PacketAction<RoomInfoList>> {
        let mut data = DATA.lock().unwrap();

        let Some(EitherData::Lobby(lobby_data)) = data.as_mut() else {
            return Ok(Default::default());
        };

        for (key, room) in &rooms.games {
            // todo
            if room.removed == Some(true) {
                lobby_data.matches.retain(|item| &item.key != key);
            } else {
                let new_match = LobbyMatch {
                    key: key.clone(),
                    players: room.player_count.unwrap_or_default(),
                    max_players: room.max_players.unwrap_or_default(),
                    name: room
                        .custom_properties
                        .get("matchname")
                        .and_then(|val| val.clone().try_into().ok())
                        .or_else(|| {
                            room.custom_properties
                                .get("roomID")
                                .and_then(|val| val.clone().try_into().ok())
                        })
                        .unwrap_or_default(),
                    map: room
                        .custom_properties
                        .get("custom_map_name")
                        .and_then(|val| val.clone().try_into().ok())
                        .or_else(|| {
                            room.custom_properties
                                .get("mapName")
                                .and_then(|val| val.clone().try_into().ok())
                        })
                        .unwrap_or_default(),
                    platform: room
                        .custom_properties
                        .get("platform")
                        .and_then(|val| val.clone().try_into().ok())
                        .or_else(|| {
                            room.custom_properties
                                .get("storeID")
                                .and_then(|val| val.clone().try_into().ok())
                        })
                        .unwrap_or_default(),
                    version: room
                        .custom_properties
                        .get("gameVersion")
                        .and_then(|val| val.clone().try_into().ok())
                        .or_else(|| {
                            room.custom_properties
                                .get("gameversion")
                                .and_then(|val| val.clone().try_into().ok())
                        })
                        .unwrap_or_default(),
                };

                // try to find existing
                match lobby_data.matches.iter_mut().find(|item| &item.key == key) {
                    Some(existing) => {
                        *existing = new_match;
                    }
                    None => {
                        lobby_data.matches.push(new_match);
                    }
                }
            }
        }

        Ok(Default::default())
    }
}
