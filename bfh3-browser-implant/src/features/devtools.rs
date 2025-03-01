use anyhow::Context as _;
use photon_lib::{
    photon::message::{PhotonMessage, PhotonMessageType},
    pun::lifting::{
        ParseEventExt as _, ParseOperationRequestExt as _, ParseOperationResponseExt as _,
    },
};

use crate::{
    DevtoolsMessage,
    networking::{PacketAction, PacketDirection, SocketType},
};

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

        let msg_bytes = rmp_serde::to_vec_named(msg).context("serialize messagepack")?;

        let (msg_type, parsed_bytes) = match msg {
            PhotonMessage::Init => (PhotonMessageType::Init, None),
            PhotonMessage::InitResponse => (PhotonMessageType::InitResponse, None),
            PhotonMessage::OperationRequest(arg) => (
                PhotonMessageType::OperationRequest,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse OperationRequest message")
                        .and_then(|p| rmp_serde::to_vec_named(&p).context("serialize messagepack")),
                ),
            ),
            PhotonMessage::OperationResponse(arg) => (
                PhotonMessageType::OperationResponse,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse OperationResponse message")
                        .and_then(|p| rmp_serde::to_vec_named(&p).context("serialize messagepack")),
                ),
            ),
            PhotonMessage::EventData(arg) => (
                PhotonMessageType::EventData,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse EventData message")
                        .and_then(|p| rmp_serde::to_vec_named(&p).context("serialize messagepack")),
                ),
            ),
            PhotonMessage::DisconnectMessage(_) => (PhotonMessageType::DisconnectMessage, None),
            PhotonMessage::InternalOperationRequest(arg) => (
                PhotonMessageType::InternalOperationRequest,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse InternalOperationRequest message")
                        .and_then(|p| rmp_serde::to_vec_named(&p).context("serialize messagepack")),
                ),
            ),
            PhotonMessage::InternalOperationResponse(arg) => (
                PhotonMessageType::InternalOperationResponse,
                Some(
                    arg.clone()
                        .parse()
                        .context("parse InternalOperationResponse message")
                        .and_then(|p| rmp_serde::to_vec_named(&p).context("serialize messagepack")),
                ),
            ),
            PhotonMessage::Message(_) => (PhotonMessageType::Message, None),
            PhotonMessage::RawMessage(_) => (PhotonMessageType::RawMessage, None),
            PhotonMessage::PingResult(_) => (PhotonMessageType::PingResult, None),
        };

        let (parsed_bytes, error) = parsed_bytes
            .map(|res| match res {
                Ok(ok) => (Some(ok), None),
                Err(err) => (None, Some(format!("{err:?}"))),
            })
            .unwrap_or((None, None));

        let devtools_message = DevtoolsMessage {
            direction,
            socket_type,
            message_type: msg_type as u8,
            message: msg_bytes,
            parsed_message: parsed_bytes,
            error,
        };

        crate::send_message_to_devtools(devtools_message);

        Ok(PacketAction::Ignore)
    }
}
