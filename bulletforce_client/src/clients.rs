use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use photon_lib::{
    WriteError,
    photon::message::{OperationResponse, PhotonMessage},
    pun::{
        constants::{internal_operation_code, operation_code},
        lifting::{
            AppStatsEvent, AuthenticateRequest, JoinLobbyRequest, ParseEventExt,
            ParseOperationResponseExt, PingRequest, PunEvent, PunOperationResponse, RoomInfo,
        },
    },
};
use strum::IntoDiscriminant;
use tracing::{debug, trace, warn};

use crate::{
    Region,
    errors::LobbyError,
    utils::{to_internal_operation_request, to_operation_request},
};

const APP_ID: &str = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";
const PING_INTERVAL: Duration = Duration::from_secs(1);

pub struct BulletForceLobbyClient {
    settings: LobbyConnectionSettings,
    state: LobbyState,
    /// When we last received a ping response. If this is none, we sent a ping request and are
    /// waiting for a response.
    last_ping_received: Option<Instant>,
    buffered_messages: Vec<Vec<u8>>,
}

impl BulletForceLobbyClient {
    pub fn get_lobby_url(&self) -> &'static str {
        self.settings.region.get_lobby_url()
    }

    pub fn create(settings: LobbyConnectionSettings) -> Self {
        Self {
            settings,
            state: LobbyState::default(),
            last_ping_received: Some(Instant::now()), // cannot set minimum value
            buffered_messages: vec![],
        }
    }

    /// Handle incoming websocket messages
    pub fn handle_input(&mut self, mut data: &[u8]) -> Result<(), LobbyError> {
        let packet = PhotonMessage::from_websocket_bytes(&mut data)?;
        trace!("Received message: {packet:?}");

        // early exit for pongs
        if let PhotonMessage::InternalOperationResponse(OperationResponse {
            operation_code: internal_operation_code::PING,
            ..
        }) = &packet
        {
            // don't parse for now, but in the future we could
            self.last_ping_received = Some(Instant::now());
            return Ok(());
        }

        self.queue_ping_if_needed()?;

        let new_state = match &mut self.state {
            LobbyState::WaitingForInitResponse => {
                if let PhotonMessage::InitResponse = packet {
                    debug!("Received lobby InitResponse");

                    self.enqueue_sent_message(to_operation_request(
                        operation_code::AUTHENTICATE,
                        AuthenticateRequest {
                            app_version: Some(self.settings.app_version.clone()),
                            application_id: Some(APP_ID.into()),
                            region: Some("eu/*".into()),
                            user_id: Some(self.settings.user_id.clone()),

                            ..Default::default()
                        },
                    ))?;
                    Some(LobbyState::WaitingForAuthResponse { app_stats: None })
                } else {
                    warn!(
                        "Expected first packte to be InitResponse, got something else: {packet:?}"
                    );
                    None
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
                                    LobbyError::Other("auth response did not contain token".into())
                                })?
                                .try_into()
                                .map_err(|e| {
                                    LobbyError::Other(
                                        format!("failed to read auth token as string: {e}").into(),
                                    )
                                })?;

                            Some(LobbyState::ReadyNoLobby {
                                token,
                                app_stats: app_stats.clone(),
                            })
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForAuthResponse: {response:?}"
                            );
                            None
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
                    None
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                    None
                }
            },
            LobbyState::ReadyNoLobby { app_stats, .. } => match packet {
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                            None
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                            None
                        }
                    }
                }
                _ => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                    None
                }
            },
            LobbyState::JoiningLobby { app_stats, token } => match packet {
                PhotonMessage::OperationResponse(operation_response) => {
                    let (response, _return_code, _debug_str) = operation_response.parse()?;
                    match response {
                        PunOperationResponse::JoinLobby(_join_lobby) => {
                            debug!("Received lobby JoinLobby");

                            Some(LobbyState::Ready {
                                token: token.clone(),
                                app_stats: app_stats.clone(),
                                games: HashMap::new(),
                            })
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForAuthResponse: {response:?}"
                            );
                            None
                        }
                    }
                }
                PhotonMessage::EventData(event_data) => {
                    let event_data = event_data.parse()?;
                    match event_data {
                        PunEvent::AppStats(new_app_stats) => {
                            *app_stats = Some(*new_app_stats.clone());
                            None
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                            None
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                    None
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
                            None
                        }
                        PunEvent::GameList(game_list) => {
                            *games = game_list.games.into_iter().collect();
                            None
                        }
                        PunEvent::GameListUpdate(game_list) => {
                            for (key, game) in game_list.games.into_iter() {
                                if game.removed == Some(true) {
                                    games.remove(&key);
                                } else {
                                    games.insert(key, game);
                                }
                            }
                            None
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForAuthResponse: {event_data:?}");
                            None
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                    None
                }
            },
        };

        if let Some(new_state) = new_state {
            self.set_new_state(new_state);
        }

        Ok(())
    }

    /// Get a message to send out through the websocket connection
    pub fn take_messages_to_send(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.buffered_messages)
    }

    pub fn get_state(&self) -> &LobbyState {
        &self.state
    }

    pub fn queue_ping_if_needed(&mut self) -> Result<(), LobbyError> {
        let Some(last_received) = self.last_ping_received else {
            // a ping is already in-flight, waiting for a response
            return Ok(());
        };

        if last_received.elapsed() > PING_INTERVAL {
            trace!("Sending a new ping request");
            self.enqueue_sent_message(to_internal_operation_request(
                internal_operation_code::PING,
                PingRequest { client_time: 0 },
            ))?;

            self.last_ping_received = None;
        }

        Ok(())
    }

    pub fn join_lobby(&mut self) -> Result<(), LobbyError> {
        // funky take/match combo to avoid clone.
        let (app_stats, token) = match std::mem::take(&mut self.state) {
            LobbyState::ReadyNoLobby { app_stats, token } => (app_stats, token),
            state => {
                // restore state
                self.state = state;
                return Err(LobbyError::Other(
                    format!(
                        "state should be {:?} when joining lobby, was {:?}",
                        LobbyStateDiscriminants::ReadyNoLobby,
                        self.state.discriminant(),
                    )
                    .into(),
                ));
            }
        };

        debug!("Sending join lobby request");
        self.enqueue_sent_message(to_operation_request(
            operation_code::JOIN_LOBBY,
            JoinLobbyRequest {
                lobby_name: None,
                lobby_type: None,
            },
        ))?;

        self.set_new_state(LobbyState::JoiningLobby { token, app_stats });
        Ok(())
    }

    fn enqueue_sent_message(&mut self, message: PhotonMessage) -> Result<(), WriteError> {
        let mut buf = vec![];
        message.to_websocket_bytes(&mut buf)?;
        self.buffered_messages.push(buf);
        Ok(())
    }

    fn set_new_state(&mut self, new_state: LobbyState) {
        debug!(
            "Lobby state {:?} ➜ {:?}",
            self.state.discriminant(),
            new_state.discriminant()
        );
        self.state = new_state;
    }
}

pub struct LobbyConnectionSettings {
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
}
