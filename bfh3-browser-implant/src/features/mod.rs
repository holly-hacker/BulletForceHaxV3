mod devtools;
mod lobby;
mod sidepanel_data;

use photon_lib::{photon::message::PhotonMessage, pun::lifting::RoomInfoList};

use crate::networking::{PacketAction, PacketDirection, SocketType};

pub use devtools::DevtoolsMessage;
pub use sidepanel_data::{GameData, LobbyData};

// this macro will:
// - generate `pub const ALL_FEATURES: [FeatureEnum; _] = [...];`
// - impl `Feature` for `FeatureEnum`
bfh3_browser_implant_macros::gen_features_impl! {
    pub enum FeatureEnum {
        Devtools(devtools::DevtoolsFeature),
        Sidepanel(sidepanel_data::SidepanelData),
        Lobby(lobby::LobbyFeature),
    }

    pub trait Feature {
        fn get_name(&self) -> &'static str;

        fn on_tick(&self) -> anyhow::Result<()> {
            Ok(())
        }

        fn on_socket_open(&self, _socket_type: SocketType) -> anyhow::Result<()> {
            Ok(())
        }

        fn on_socket_close(&self, _socket_type: SocketType) -> anyhow::Result<()> {
            Ok(())
        }

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
}
