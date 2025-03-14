mod devtools;
mod lobby;

use photon_lib::{photon::message::PhotonMessage, pun::lifting::RoomInfoList};

use crate::networking::{PacketAction, PacketDirection, SocketType};

pub use devtools::DevtoolsMessage;

pub const ALL_FEATURES: [&dyn Feature; 3] = [
    &DummyFeature,
    &devtools::DevtoolsFeature,
    &lobby::LobbyFeature,
];

pub trait Feature {
    fn get_name(&self) -> &'static str;

    // fn on_tick() {}

    /// Triggered when the game sends or receives a packet
    fn on_packet(
        &self,
        _msg: &PhotonMessage,
        _socket: SocketType,
        _direction: PacketDirection,
    ) -> anyhow::Result<PacketAction<PhotonMessage>> {
        Ok(PacketAction::Ignore)
    }

    fn on_gamelist_update(
        &self,
        _rooms: &RoomInfoList,
    ) -> anyhow::Result<PacketAction<RoomInfoList>> {
        Ok(PacketAction::Ignore)
    }
}

struct DummyFeature;

impl Feature for DummyFeature {
    fn get_name(&self) -> &'static str {
        "dummy"
    }
}
