use anyhow::Context as _;
use photon_lib::{
    photon::message::{PhotonMessage, PhotonMessageType},
    pun::lifting::{
        ParseEventExt as _, ParseOperationRequestExt as _, ParseOperationResponseExt as _,
        PunEvent, PunOperationRequest, RaiseEventParsed, RpcCall,
    },
};
use strum::EnumProperty as _;
use wasm_bindgen::prelude::*;

use crate::{
    bindgen::send_message_to_devtools,
    game_data::BfhRpcCall,
    networking::{PacketAction, PacketDirection, SocketType},
};

#[wasm_bindgen]
pub struct DevtoolsMessage {
    /// The direction the packet was going
    pub direction: PacketDirection,
    /// Which server the socket is connected to
    pub socket_type: SocketType,
    /// The type of the message
    pub message_type: u8,
    /// The raw message, encoded as JSON
    #[wasm_bindgen(getter_with_clone)]
    pub raw_message: String,
    /// The high-level message (if any), encoded as JSON
    #[wasm_bindgen(getter_with_clone)]
    pub lifted_message: Option<String>,
    /// An interpreted version of the message (if any), encoded as JSON
    #[wasm_bindgen(getter_with_clone)]
    pub interpreted_message: Option<String>,
    /// The parsing error, if any occurred
    #[wasm_bindgen(getter_with_clone)]
    pub detail: Option<String>,
    /// Whether an error occurred parsing or lifting this message
    pub has_error: bool,
}

pub struct DevtoolsFeature;

impl super::Feature for DevtoolsFeature {
    fn get_name(&self) -> &'static str {
        "devtools"
    }

    fn on_packet(
        &self,
        msg: &PhotonMessage,
        socket_type: SocketType,
        direction: PacketDirection,
    ) -> anyhow::Result<PacketAction<PhotonMessage>> {
        // TODO: only if enabled?

        let raw_message = serde_json::to_string(msg).context("serialize JSON")?;

        let (message_type, maybe_lifted_message, maybe_interpreted_message, mut detail) = match msg
        {
            PhotonMessage::Init => (PhotonMessageType::Init, None, None, None),
            PhotonMessage::InitResponse => (PhotonMessageType::InitResponse, None, None, None),
            PhotonMessage::OperationRequest(arg) => {
                let parsed = arg
                    .clone()
                    .parse()
                    .context("parse OperationRequest message");

                let mut detail = None;

                let interpreted = parsed.as_ref().ok().and_then(|op_req| match op_req {
                    PunOperationRequest::RaiseEvent(raise_event) => {
                        let parsed_res = RaiseEventParsed::try_from(*raise_event.clone())
                            .context("try into parsed event");
                        if let Ok(parsed) = &parsed_res {
                            let name: &'static str = (&parsed.data).into();
                            detail = Some(format!("Event: {name}"));

                            if let PunEvent::Rpc(rpc) = &parsed.data {
                                if let Some(call) = &rpc.data {
                                    detail = Some(get_rpc_function_call_string(call));
                                }
                            }
                        }
                        Some(parsed_res)
                    }
                    _ => None,
                });

                (
                    PhotonMessageType::OperationRequest,
                    Some(parsed.and_then(|p| serde_json::to_string(&p).context("serialize JSON"))),
                    interpreted.map(|res| {
                        res.and_then(|ev| serde_json::to_string(&ev).context("serialize JSON"))
                    }),
                    detail,
                )
            }
            PhotonMessage::OperationResponse(arg) => (
                PhotonMessageType::OperationResponse,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse OperationResponse message")
                        .and_then(|p| serde_json::to_string(&p).context("serialize JSON")),
                ),
                None,
                None,
            ),
            PhotonMessage::EventData(arg) => {
                let parsed = arg.clone().parse().context("parse EventData message");

                let mut detail = None;

                if let Ok(PunEvent::Rpc(rpc)) = parsed.as_ref() {
                    if let Some(call) = &rpc.data {
                        detail = Some(get_rpc_function_call_string(call));
                    }
                }

                (
                    PhotonMessageType::EventData,
                    Some(parsed.and_then(|p| serde_json::to_string(&p).context("serialize JSON"))),
                    None,
                    detail,
                )
            }
            PhotonMessage::DisconnectMessage(_) => {
                (PhotonMessageType::DisconnectMessage, None, None, None)
            }
            PhotonMessage::InternalOperationRequest(arg) => (
                PhotonMessageType::InternalOperationRequest,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse InternalOperationRequest message")
                        .and_then(|p| serde_json::to_string(&p).context("serialize JSON")),
                ),
                None,
                None,
            ),
            PhotonMessage::InternalOperationResponse(arg) => (
                PhotonMessageType::InternalOperationResponse,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse InternalOperationResponse message")
                        .and_then(|p| serde_json::to_string(&p).context("serialize JSON")),
                ),
                None,
                None,
            ),
            PhotonMessage::Message(_) => (PhotonMessageType::Message, None, None, None),
            PhotonMessage::RawMessage(_) => (PhotonMessageType::RawMessage, None, None, None),
            PhotonMessage::PingResult(_) => (PhotonMessageType::PingResult, None, None, None),
        };

        let mut has_error = false;

        let lifted_message = match maybe_lifted_message {
            None => None,
            Some(Ok(lifted_message)) => Some(lifted_message),
            Some(Err(err)) => {
                has_error = true;
                detail = Some(format!("{err:?}"));
                None
            }
        };

        let interpreted_message = match maybe_interpreted_message {
            None => None,
            Some(Ok(interpreted_message)) => Some(interpreted_message),
            Some(Err(err)) => {
                has_error = true;
                detail = Some(format!("{err:?}"));
                None
            }
        };

        let devtools_message = DevtoolsMessage {
            direction,
            socket_type,
            message_type: message_type as u8,
            raw_message,
            lifted_message,
            interpreted_message,
            detail,
            has_error,
        };

        send_message_to_devtools(devtools_message);

        Ok(PacketAction::Ignore)
    }
}

fn get_rpc_function_call_string(call: &RpcCall) -> String {
    let rpc_name = match (call.rpc_index, &call.method_name) {
        (Some(index), _) => BfhRpcCall::from_repr(index)
            .map(|call| {
                call.get_str("Name")
                    .unwrap_or_else(|| call.into())
                    .to_owned()
            })
            .unwrap_or_else(|| "<unknown>".to_string()),
        (_, Some(name)) => name.clone(),
        _ => "<unknown>".into(),
    };

    let mut args = String::new();
    if let Some(items) = &call.in_method_parameters {
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&item.to_string());
        }
    }

    format!("RPC call: {rpc_name}({args})")
}
