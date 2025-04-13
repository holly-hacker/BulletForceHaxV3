use std::{borrow::Cow, collections::HashMap};

use photon_lib::{
    photon::message::PhotonMessage,
    pun::{
        constants::operation_code,
        lifting::{
            AppStatsEvent, AuthenticateRequest, JoinGameRequest, JoinLobbyRequest,
            ParseEventExt as _, ParseOperationResponseExt as _, PunEvent, PunOperationResponse,
            RoomInfo,
        },
    },
};
use strum::IntoDiscriminant;
use tracing::{debug, trace, warn};

use crate::{Region, errors::HandlerError, utils::to_operation_request};

use super::{Client, ClientContext, ClientImpl};

const APP_ID: &str = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";

#[derive(Default)]
pub struct LobbyClient;

impl ClientImpl for LobbyClient {
    type Settings = LobbyClientSettings;
    type State = LobbyState;

    fn get_url(&self, ctx: &ClientContext<Self>) -> Cow<'static, str> {
        ctx.settings.region.get_lobby_url().into()
    }

    fn handle_incoming_packet(
        &mut self,
        ctx: &mut ClientContext<LobbyClient>,
        packet: PhotonMessage,
    ) -> Result<(), HandlerError> {
        match &mut ctx.state {
            LobbyState::WaitingForInitResponse => {
                if let PhotonMessage::InitResponse = packet {
                    debug!("Received lobby InitResponse");

                    ctx.enqueue_sent_message(to_operation_request(
                        operation_code::AUTHENTICATE,
                        AuthenticateRequest {
                            app_version: Some(ctx.settings.app_version.clone()),
                            application_id: Some(APP_ID.into()),
                            region: Some("eu/*".into()),
                            user_id: Some(ctx.settings.user_id.clone()),

                            ..Default::default()
                        },
                    ))?;
                    ctx.set_new_state(LobbyState::WaitingForAuthResponse { app_stats: None })
                } else {
                    warn!(
                        "Expected first packte to be InitResponse, got something else: {packet:?}"
                    );
                }
            }
            LobbyState::WaitingForAuthResponse { app_stats } => match packet {
                PhotonMessage::OperationResponse(operation_response) => {
                    let (response, _return_code, _debug_str) = operation_response.parse()?;
                    match response {
                        PunOperationResponse::Authenticate(authenticate) => {
                            debug!("Received lobby authenticate response");

                            let token: String = authenticate
                                .token
                                .ok_or_else(|| {
                                    HandlerError::Other(
                                        "auth response did not contain token".into(),
                                    )
                                })?
                                .try_into()
                                .map_err(|e| {
                                    HandlerError::Other(
                                        format!("failed to read auth token as string: {e}").into(),
                                    )
                                })?;

                            let app_stats = app_stats.clone();
                            ctx.set_new_state(LobbyState::ReadyNoLobby { token, app_stats })
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForAuthResponse: {response:?}"
                            );
                        }
                    }
                }
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                }
            },
            LobbyState::ReadyNoLobby { app_stats, .. } => match packet {
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                        }
                    }
                }
                _ => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                }
            },
            LobbyState::JoiningLobby { app_stats, token } => match packet {
                PhotonMessage::OperationResponse(operation_response) => {
                    let (response, _return_code, _debug_str) = operation_response.parse()?;
                    match response {
                        PunOperationResponse::JoinLobby(_join_lobby) => {
                            debug!("Received lobby JoinLobby");

                            let token = token.clone();
                            let app_stats = app_stats.clone();
                            ctx.set_new_state(LobbyState::Ready {
                                token,
                                app_stats,
                                games: HashMap::new(),
                            })
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForAuthResponse: {response:?}"
                            );
                        }
                    }
                }
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                }
            },
            LobbyState::Ready {
                app_stats, games, ..
            } => match packet {
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                        }
                        PunEvent::GameList(game_list) => {
                            *games = game_list.games.into_iter().collect();
                        }
                        PunEvent::GameListUpdate(game_list) => {
                            for (key, game) in game_list.games.into_iter() {
                                if game.removed == Some(true) {
                                    games.remove(&key);
                                } else {
                                    games.insert(key, game);
                                }
                            }
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                }
            },
            LobbyState::JoiningGame { token, room_name } => {
                match packet {
                    PhotonMessage::OperationResponse(op_resp) => {
                        let (op_resp, _, _) = op_resp.parse()?;

                        if let PunOperationResponse::JoinGame(join_game) = op_resp {
                            let Some(address) = join_game.address else {
                                return Err(HandlerError::Other(
                                    "JoinGame response did not contain an address!".into(),
                                ));
                            };

                            let token = token.clone();
                            let room_name = room_name.clone();
                            ctx.set_new_state(LobbyState::ReadyToJoinGame {
                                token,
                                room_name,
                                address,
                            });
                        }
                    }
                    _ => {
                        // do nothing
                        trace!("Discarding incoming message because we're joining a game");
                    }
                }
            }
            LobbyState::ReadyToJoinGame { .. } => {
                // don't do anything, the user is meant to take over at this point
                trace!("Discarding incoming message because we're ready to join a game");
            }
        }

        Ok(())
    }
}

impl Client<LobbyClient> {
    pub fn join_lobby(&mut self) -> Result<(), HandlerError> {
        // funky take/match combo to avoid clone.
        let (app_stats, token) = match std::mem::take(&mut self.context.state) {
            LobbyState::ReadyNoLobby { app_stats, token } => (app_stats, token),
            state => {
                // restore state
                self.context.state = state;
                return Err(HandlerError::Other(
                    format!(
                        "state should be {:?} when joining lobby, was {:?}",
                        LobbyStateDiscriminants::ReadyNoLobby,
                        self.context.state.discriminant(),
                    )
                    .into(),
                ));
            }
        };

        debug!("Sending join lobby request");
        self.context.enqueue_sent_message(to_operation_request(
            operation_code::JOIN_LOBBY,
            JoinLobbyRequest {
                lobby_name: None,
                lobby_type: None,
            },
        ))?;

        self.context
            .set_new_state(LobbyState::JoiningLobby { token, app_stats });

        Ok(())
    }

    pub fn join_game(&mut self, room_name: String) -> Result<(), HandlerError> {
        let token = match std::mem::take(&mut self.context.state) {
            LobbyState::Ready { token, .. } => token,
            state => {
                // restore state
                self.context.state = state;
                return Err(HandlerError::Other(
                    format!(
                        "state should be {:?} when joining game, was {:?}",
                        LobbyStateDiscriminants::ReadyNoLobby,
                        self.context.state.discriminant(),
                    )
                    .into(),
                ));
            }
        };

        debug!("Sending join game request");
        self.context.enqueue_sent_message(to_operation_request(
            operation_code::JOIN_GAME,
            JoinGameRequest {
                room_name: Some(room_name.clone()),
                ..Default::default()
            },
        ))?;

        self.context
            .set_new_state(LobbyState::JoiningGame { token, room_name });

        Ok(())
    }
}

pub struct LobbyClientSettings {
    pub app_version: String,
    pub user_id: String,
    pub region: Region,
}

#[derive(Default, strum::EnumDiscriminants)]
pub enum LobbyState {
    #[default]
    WaitingForInitResponse,
    WaitingForAuthResponse {
        app_stats: Option<AppStatsEvent>,
    },
    ReadyNoLobby {
        token: String,
        app_stats: Option<AppStatsEvent>,
    },
    JoiningLobby {
        token: String,
        app_stats: Option<AppStatsEvent>,
    },
    Ready {
        token: String,
        app_stats: Option<AppStatsEvent>,
        games: HashMap<String, RoomInfo>,
    },
    JoiningGame {
        token: String,
        room_name: String,
    },
    ReadyToJoinGame {
        token: String,
        room_name: String,
        address: String,
    },
}
