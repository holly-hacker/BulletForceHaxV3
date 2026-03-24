use std::collections::HashMap;

use photon_bulletforce::rpc::BfhRpcCall;
use photon_lib::{
    PhotonHashmap, PhotonObject,
    photon::message::PhotonMessage,
    pun::{
        ViewId,
        constants::operation_code,
        lifting::{
            ActorInfo, AuthenticateRequest, JoinGameRequest, ParseEventExt as _,
            ParseOperationResponseExt as _, PunEvent, PunOperationRequest, PunOperationResponse,
            RaiseEventParsed, RoomInfo, RpcCall, RpcEvent, SetPropertiesRequest,
        },
    },
};
use tracing::{debug, warn};

use crate::{errors::HandlerError, utils::to_operation_request};

use super::{Client, ClientImpl};

#[derive(Default)]
pub struct GameClient;

impl ClientImpl for GameClient {
    type Settings = GameClientSettings;
    type State = GameState;

    fn get_url(&self, ctx: &super::ClientContext<Self>) -> std::borrow::Cow<'static, str> {
        ctx.settings.address.clone().into()
    }

    fn handle_incoming_packet(
        &mut self,
        ctx: &mut super::ClientContext<Self>,
        packet: PhotonMessage,
    ) -> Result<(), crate::errors::HandlerError> {
        match &mut ctx.state {
            GameState::WaitingForInitResponse => {
                if let PhotonMessage::InitResponse = packet {
                    debug!("Received game InitResponse");

                    ctx.enqueue_sent_message(to_operation_request(
                        operation_code::AUTHENTICATE,
                        AuthenticateRequest {
                            token: Some(PhotonObject::String(ctx.settings.token.clone())),
                            ..Default::default()
                        },
                    ))?;
                    ctx.set_new_state(GameState::WaitingForAuthResponse);
                } else {
                    warn!(
                        "Expected first packte to be InitResponse, got something else: {packet:?}"
                    );
                }
            }
            GameState::WaitingForAuthResponse => match packet {
                PhotonMessage::OperationResponse(operation_response) => {
                    let (response, _return_code, _debug_str) = operation_response.parse()?;
                    match response {
                        PunOperationResponse::Authenticate(_authenticate) => {
                            debug!("Received lobby authenticate response");

                            ctx.enqueue_sent_message(to_operation_request(
                                operation_code::JOIN_GAME,
                                JoinGameRequest {
                                    room_name: Some(ctx.settings.room_name.clone()),
                                    broadcast: Some(true),
                                    // TODO: send player info
                                    player_properties: Some(
                                        photon_lib::indexmap::indexmap! {
                                            PhotonObject::Byte(0) => PhotonObject::String("v9test".into())
                                        }
                                        .into(),
                                    ),

                                    ..Default::default()
                                },
                            ))?;

                            ctx.set_new_state(GameState::WaitingForJoinGameResponse);
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForAuthResponse: {response:?}"
                            );
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForAuthResponse phase: {packet:?}");
                }
            },
            GameState::WaitingForJoinGameResponse => match packet {
                PhotonMessage::OperationResponse(op_resp) => {
                    let (resp, _return_code, _debug_str) = op_resp.parse()?;
                    match resp {
                        PunOperationResponse::JoinGame(join_game) => {
                            debug!(actor_nr = join_game.actor_nr, "Received join game response");
                            let room_info = join_game.game_properties.unwrap();
                            let player_properties = join_game
                                .player_properties
                                .unwrap()
                                .into_iter()
                                .flat_map(|(k, v)| {
                                    let PhotonObject::Integer(k) = k else {
                                        return None;
                                    };
                                    Some((k, v))
                                })
                                .collect::<HashMap<_, _>>();

                            ctx.set_new_state(GameState::WaitingForJoinEvent {
                                actor_nr: join_game.actor_nr.unwrap(),
                                room_info: Box::new(room_info),
                                player_properties,
                            });
                        }
                        _ => {
                            warn!(
                                "Unexpected operation response in WaitingForJoinGameResponse: {resp:?}"
                            );
                        }
                    }
                }
                packet => {
                    warn!(
                        "Unexpected message type in WaitingForJoinGameResponse phase: {packet:?}"
                    );
                }
            },
            GameState::WaitingForJoinEvent {
                actor_nr,
                room_info,
                player_properties,
            } => match packet {
                PhotonMessage::EventData(event) => {
                    let event = event.parse()?;
                    match event {
                        PunEvent::Join(join) => {
                            debug!(actor_nr = join.actor_nr, "Received join event");
                            let actor_nr = *actor_nr;
                            let room_info = room_info.clone();
                            let player_properties = player_properties.clone();
                            ctx.set_new_state(GameState::Ready {
                                actor_nr,
                                room_info,
                                player_properties,
                            });
                        }
                        _ => {
                            warn!("Unexpected event in WaitingForJoinEvent: {event:?}");
                        }
                    }
                }
                packet => {
                    warn!("Unexpected message type in WaitingForJoinEvent phase: {packet:?}");
                }
            },
            GameState::Ready {
                room_info,
                player_properties,
                ..
            } => match packet {
                PhotonMessage::EventData(event) => match event.parse()? {
                    PunEvent::PropertiesChanged(props_changed) => {
                        // TODO: creating a full copy of the hashmap is kinda hacky and slow, but
                        // it's the easiest way for now
                        if props_changed.target_actor_number == 0 {
                            debug!(
                                prop_count = props_changed.properties.0.len(),
                                "Updating room"
                            );
                            // update room
                            let mut existing_hashmap: PhotonHashmap =
                                room_info.as_ref().clone().into();
                            props_changed.properties.0.into_iter().for_each(|(k, v)| {
                                existing_hashmap.0.insert(k, v);
                            });
                            let new_room_info: RoomInfo = existing_hashmap.try_into()?;
                            **room_info = new_room_info;
                        } else {
                            debug!(
                                actor_id = props_changed.target_actor_number,
                                prop_count = props_changed.properties.0.len(),
                                "Updating actor"
                            );
                            // update actor
                            let actor_info_ref = player_properties
                                .get_mut(&props_changed.target_actor_number)
                                .ok_or_else(|| {
                                    HandlerError::Other(
                                        "actor for PropertiesChanged not found".into(),
                                    )
                                })?;

                            let mut existing_hashmap: PhotonHashmap = actor_info_ref.clone().into();
                            props_changed.properties.0.into_iter().for_each(|(k, v)| {
                                existing_hashmap.0.insert(k, v);
                            });
                            *actor_info_ref = existing_hashmap.try_into()?;
                        }
                    }
                    PunEvent::SendSerialize(send_serialize) => {
                        let serialized = send_serialize.get_serialized_data();
                        debug!("SendSerialize: {serialized:?}");
                    }
                    ev => {
                        debug!("unhandled event: {ev:?}");
                    }
                },
                _ => {
                    debug!("unhandled packet type: {packet:?}");
                }
            },
        }

        Ok(())
    }
}

pub struct GameClientSettings {
    pub token: String,
    pub room_name: String,
    pub address: String,
}

#[derive(Default, strum::EnumDiscriminants)]
pub enum GameState {
    #[default]
    WaitingForInitResponse,
    WaitingForAuthResponse,
    WaitingForJoinGameResponse,
    WaitingForJoinEvent {
        actor_nr: i32,
        room_info: Box<RoomInfo>,
        player_properties: HashMap<i32, ActorInfo>,
    },
    Ready {
        actor_nr: i32,
        room_info: Box<RoomInfo>,
        player_properties: HashMap<i32, ActorInfo>,
    },
}

pub struct PlayerInfo {
    pub player_properties: ActorInfo,
}

impl Client<GameClient> {
    pub fn set_player_properties(
        &mut self,
        properties: impl Into<PhotonHashmap>,
    ) -> Result<(), HandlerError> {
        let GameState::Ready { actor_nr, .. } = self.get_state() else {
            return Err(HandlerError::Other(
                "invalid state, should be Ready. return error here".into(),
            ));
        };

        let req = SetPropertiesRequest {
            properties: properties.into(),
            actor_nr: Some(*actor_nr),
            broadcast: Some(true),
            event_forward: None,
        };
        let pun_op_req = PunOperationRequest::SetProperties(Box::new(req));
        let op_req = pun_op_req.unparse();
        let message = PhotonMessage::OperationRequest(op_req);
        self.context.enqueue_sent_message(message)?;

        Ok(())
    }

    pub fn send_rpc_call(
        &mut self,
        call: BfhRpcCall,
        args: &[PhotonObject],
    ) -> Result<(), HandlerError> {
        let GameState::Ready { actor_nr, .. } = self.get_state() else {
            return Err(HandlerError::Other(
                "invalid state, should be Ready. return error here".into(),
            ));
        };

        let actor_nr = *actor_nr;

        self.raise_event(RaiseEventParsed {
            cache: Some(4),
            data: PunEvent::Rpc(Box::new(RpcEvent {
                sender_actor: Some(actor_nr),
                data: Some(RpcCall {
                    rpc_index: Some(call as u8),
                    net_view_id: ViewId(actor_nr * 1000 + 1),
                    server_timestamp: None,
                    in_method_parameters: Some(args.to_vec()),
                    other_side_prefix: None,
                    method_name: None,
                    custom_properties: Default::default(),
                }),
            })),
            actor_list: None,
            group: None,
            receiver_group: None,
            event_forward: None,
        })?;

        Ok(())
    }
}
