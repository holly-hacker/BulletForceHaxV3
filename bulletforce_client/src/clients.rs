use photon_lib::{
    photon::message::PhotonMessage,
    pun::{
        constants::operation_code,
        lifting::{
            AppStatsEvent, AuthenticateRequest, JoinLobbyRequest, ParseEventExt,
            ParseOperationResponseExt, PunEvent, PunOperationResponse,
        },
    },
};
use strum::IntoDiscriminant;
use tracing::{debug, trace, warn};

use crate::utils::to_operation_request;

const APP_ID: &str = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";

pub struct BulletForceLobbyClient {
    settings: LobbyConnectionSettings,
    state: LobbyState,
    buffered_messages: Vec<Vec<u8>>,
}

impl BulletForceLobbyClient {
    pub fn get_lobby_server() -> &'static str {
        // TODO: support different servers
        "wss://game-ca-1.blayzegames.com:2053/?libversion=4.1.6.10&sid=30&app="
    }

    pub fn create(settings: LobbyConnectionSettings) -> Self {
        Self {
            settings,
            state: LobbyState::default(),
            buffered_messages: vec![],
        }
    }

    /// Handle incoming websocket messages
    pub fn handle_input(&mut self, mut data: &[u8]) {
        let packet = PhotonMessage::from_websocket_bytes(&mut data).expect("parse message");
        trace!("Received message: {packet:?}");

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
                    ));
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
                    let (response, _return_code, _debug_str) = operation_response.parse().unwrap();
                    match response {
                        PunOperationResponse::Authenticate(authenticate) => {
                            debug!("Received lobby authenticate response");

                            let token: String = authenticate
                                .token
                                .expect("response should contain token")
                                .try_into()
                                .expect("token should be string");

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
                    let event_data = event_data.parse().unwrap();
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
                    let event_data = event_data.parse().unwrap();
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
                    let (response, _return_code, _debug_str) = operation_response.parse().unwrap();
                    match response {
                        PunOperationResponse::JoinLobby(_join_lobby) => {
                            debug!("Received lobby JoinLobby");

                            Some(LobbyState::Ready {
                                token: token.clone(),
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
                    let event_data = event_data.parse().unwrap();
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
            LobbyState::Ready { .. } => {
                debug!("Received packet in Ready state");
                None
            }
        };

        if let Some(new_state) = new_state {
            debug!(
                "Lobby state {:?} ➜ {:?}",
                self.state.discriminant(),
                new_state.discriminant()
            );
            self.state = new_state;
        }
    }

    /// Get a message to send out through the websocket connection
    pub fn take_messages_to_send(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.buffered_messages)
    }

    pub fn get_state(&self) -> &LobbyState {
        &self.state
    }

    pub fn join_lobby(&mut self) {
        // funky take/match combo to avoid clone.
        let (app_stats, token) = match std::mem::take(&mut self.state) {
            LobbyState::ReadyNoLobby { app_stats, token } => (app_stats, token),
            state => {
                // restore state
                self.state = state;
                todo!("handle error due to bad state");
            }
        };

        debug!("Sending join lobby request");
        self.enqueue_sent_message(to_operation_request(
            operation_code::JOIN_LOBBY,
            JoinLobbyRequest {
                lobby_name: None,
                lobby_type: None,
            },
        ));

        self.state = LobbyState::JoiningLobby { token, app_stats }
    }

    fn enqueue_sent_message(&mut self, message: PhotonMessage) {
        let mut buf = vec![];
        message.to_websocket_bytes(&mut buf).unwrap();
        self.buffered_messages.push(buf);
    }
}

pub struct LobbyConnectionSettings {
    pub app_version: String,
    pub user_id: String,
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
    },
}
