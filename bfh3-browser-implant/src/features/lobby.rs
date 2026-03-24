use photon_lib::{photon::object::PhotonObject, pun::lifting::RoomInfoList};

use crate::networking::PacketAction;

pub struct LobbyFeature;

impl super::Feature for LobbyFeature {
    fn get_name(&self) -> &'static str {
        "lobby"
    }

    fn on_gamelist_update(
        &self,
        rooms: &RoomInfoList,
    ) -> anyhow::Result<PacketAction<RoomInfoList>> {
        let mut rooms = rooms.clone();

        for (_, room) in &mut rooms.games {
            let mut changed_pw = false;

            if let Some(pw) = room.custom_properties.get_mut("password")
                && let PhotonObject::String(pw_str) = pw
                && !pw_str.is_empty()
            {
                *pw = PhotonObject::Null;
                changed_pw = true;
            }

            if changed_pw
                && let Some(PhotonObject::String(room_name)) =
                    room.custom_properties.get_mut("roomName")
            {
                *room_name = format!("[pw] {room_name}");
            }
        }

        Ok(PacketAction::Modify(rooms))
    }
}
