use std::convert::TryInto;

use anyhow::Context;
use photon_lib::{
    // parsing::event::ParseEventExt,
    photon::message::{DisconnectMessage, OperationRequest, OperationResponse, PhotonMessage},
    pun::lifting::{
        ParseEventExt as _, ParseOperationRequestExt as _, ParseOperationResponseExt as _, PunEvent,
    },
};
use tracing::{error, trace, warn};

use crate::features::ALL_FEATURES;

pub fn handle_packet_send(buffer: &[u8], url: &str) -> anyhow::Result<PacketAction<Vec<u8>>> {
    // get port from url
    trace!(url, "extracting port from url");
    let port_str = url.split(':').last().unwrap().split('/').next().unwrap();

    let socket_type: SocketType = port_str
        .parse::<u16>()
        .context("parse port as u16")?
        .try_into()
        .map_err(|e| anyhow::anyhow!("convert port to socket type: {e}"))?;

    handle_packet_message(buffer, PacketDirection::Send, socket_type)
}

pub fn handle_packet_recv(buffer: &[u8], origin: &str) -> anyhow::Result<PacketAction<Vec<u8>>> {
    // get port from origin
    trace!(origin, "extracting port from origin");
    let port_str = origin.split(':').last().unwrap();

    let socket_type: SocketType = port_str
        .parse::<u16>()
        .context("parse port as u16")?
        .try_into()
        .map_err(|e| anyhow::anyhow!("convert port to socket type: {e}"))?;

    handle_packet_message(buffer, PacketDirection::Recv, socket_type)
}

#[tracing::instrument(skip(buffer), level = "debug")]
fn handle_packet_message(
    buffer: &[u8],
    direction: PacketDirection,
    socket_type: SocketType,
) -> anyhow::Result<PacketAction<Vec<u8>>> {
    trace!("Received bytes: {buffer:?}");
    let msg = PhotonMessage::from_websocket_bytes(&mut buffer.iter().as_slice())
        .context("parse ws message")?;
    trace!("Parsed photon message: {msg:?}");

    let res = dispatch_to_features(msg, direction, socket_type)
        .context("dispatch photon message to features")?;

    let mapped = res
        .map(|msg| {
            let mut ret = Vec::new();
            msg.to_websocket_bytes(&mut ret)
                .context("serialize new packet")
                .map(|()| ret)
        })
        .transpose()?;

    Ok(mapped)
}

fn dispatch_to_features(
    msg: PhotonMessage,
    direction: PacketDirection,
    socket_type: SocketType,
) -> anyhow::Result<PacketAction<PhotonMessage>> {
    let top_level_action = ALL_FEATURES
        .iter()
        .try_fold(PacketAction::Ignore, |acc, feat| {
            acc.fold_with(&msg, |m| feat.on_packet(m, socket_type, direction))
                .with_context(|| format!("run for feat: {}", feat.get_name()))
        })
        .context("run on_packet handler")?;

    let (msg, is_drop) = match top_level_action {
        PacketAction::Ignore => (msg, false),
        PacketAction::Modify(modified_msg) => (modified_msg, false),
        PacketAction::Drop => (msg, true),
    };

    let res = match msg {
        PhotonMessage::Init => Ok(PacketAction::Ignore),
        PhotonMessage::InitResponse => Ok(PacketAction::Ignore),
        PhotonMessage::OperationRequest(operation_request) => {
            let clone = operation_request.parameters.clone();
            match operation_request.parse() {
                Ok(parsed) => {
                    trace!("OperationRequest: {parsed:?}");
                }
                Err(e) => {
                    warn!("failed to parse operation request {e}: {clone:?}");
                }
            }
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::OperationResponse(operation_response) => {
            let clone = operation_response.parameters.clone();
            match operation_response.parse() {
                Ok((parsed, return_code, debug_message)) => {
                    trace!(return_code, debug_message, "OperationResponse: {parsed:?}");
                }
                Err(e) => {
                    warn!("failed to parse operation response {e}: {clone:?}");
                }
            }
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::EventData(event_data) => {
            let clone = event_data.parameters.clone();
            match event_data.parse() {
                Ok(parsed) => {
                    trace!("Event: {parsed:?}");
                    match parsed {
                        PunEvent::GameList(game_list) => Ok(ALL_FEATURES
                            .iter()
                            .try_fold(PacketAction::Ignore, |acc, feat| {
                                acc.fold_with(game_list.as_ref(), |m| feat.on_gamelist_update(m))
                                    .with_context(|| format!("run for feat: {}", feat.get_name()))
                            })
                            .context("run on_gamelist_update handler")?
                            .map(|i| PunEvent::GameList(Box::new(i)).unparse())
                            .map(PhotonMessage::EventData)),
                        PunEvent::GameListUpdate(game_list) => Ok(ALL_FEATURES
                            .iter()
                            .try_fold(PacketAction::Ignore, |acc, feat| {
                                acc.fold_with(game_list.as_ref(), |m| feat.on_gamelist_update(m))
                                    .with_context(|| format!("run for feat: {}", feat.get_name()))
                            })
                            .context("run on_gamelist_update handler")?
                            .map(|i| PunEvent::GameListUpdate(Box::new(i)).unparse())
                            .map(PhotonMessage::EventData)),
                        _ => Ok(PacketAction::Ignore),
                    }
                }
                Err(e) => {
                    warn!("failed to parse event {e}: {clone:?}");
                    Ok(PacketAction::Ignore)
                }
            }
        }
        PhotonMessage::DisconnectMessage(DisconnectMessage {
            code,
            debug_message,
            parameters,
        }) => {
            trace!(code, debug_message, "Disconnect: {parameters:?}");
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::InternalOperationRequest(OperationRequest {
            operation_code,
            parameters,
        }) => {
            trace!(operation_code, "InternalOperationRequest: {parameters:?}");
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::InternalOperationResponse(OperationResponse {
            operation_code,
            return_code,
            debug_message,
            parameters,
        }) => {
            trace!(
                operation_code,
                return_code, debug_message, "InternalOperationResponse: {parameters:?}"
            );
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::Message(photon_object_type) => {
            trace!("Message: {photon_object_type:?}");
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::RawMessage(items) => {
            trace!("RawMessage: {items:?}");
            Ok(PacketAction::Ignore)
        }
        PhotonMessage::PingResult(ping_result) => {
            trace!("PingResult: {ping_result:?}");
            Ok(PacketAction::Ignore)
        }
    };

    if is_drop && matches!(res, Ok(PacketAction::Modify(_))) {
        error!("Tried to modify a parsed packet after the unparsed packet was dropped");
        return Ok(PacketAction::Drop);
    }

    res
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Send,
    Recv,
}

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Lobby,
    Game,
}

impl std::convert::TryFrom<u16> for SocketType {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2053 => Ok(Self::Lobby),
            2083 => Ok(Self::Game),
            x => Err(x),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Default)]
pub enum PacketAction<T> {
    /// Leave a packet intact
    #[default]
    Ignore,
    /// Drop a packet, causing it to not be sent
    Drop,
    /// Modify a packet, causing different data to be sent/received instead
    Modify(T),
}

impl<T> PacketAction<T> {
    pub fn fold_with<F>(self, orig_t_ref: &T, next: F) -> anyhow::Result<Self>
    where
        F: FnOnce(&T) -> anyhow::Result<Self>,
    {
        Ok(match self {
            PacketAction::Ignore => next(orig_t_ref)?,
            PacketAction::Modify(current_t) => match next(&current_t)? {
                PacketAction::Ignore => PacketAction::Modify(current_t),
                PacketAction::Modify(new_t) => PacketAction::Modify(new_t),
                PacketAction::Drop => anyhow::bail!("cannot drop modified message"),
            },
            PacketAction::Drop => match next(orig_t_ref)? {
                action @ (PacketAction::Ignore | PacketAction::Drop) => action,
                PacketAction::Modify(_) => anyhow::bail!("cannot modify dropped message"),
            },
        })
    }

    pub fn map<U, F>(self, f: F) -> PacketAction<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            PacketAction::Ignore => PacketAction::Ignore,
            PacketAction::Drop => PacketAction::Drop,
            PacketAction::Modify(a) => PacketAction::Modify(f(a)),
        }
    }
}

impl<T, E> PacketAction<Result<T, E>> {
    pub fn transpose(self) -> Result<PacketAction<T>, E> {
        match self {
            PacketAction::Ignore => Ok(PacketAction::Ignore),
            PacketAction::Drop => Ok(PacketAction::Drop),
            PacketAction::Modify(Ok(res)) => Ok(PacketAction::Modify(res)),
            PacketAction::Modify(Err(err)) => Err(err),
        }
    }
}
