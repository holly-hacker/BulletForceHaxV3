use indexmap::IndexMap;

use crate::{
    PhotonArray, PhotonHashmap, PhotonObjectArray,
    photon::{
        message::{EventData, PhotonMessageType},
        object::{CustomData, PhotonObject},
    },
    pun::{LiftingError, ViewId, constants::*},
};

pub trait ParseEventExt {
    fn parse(self) -> Result<PunEvent, LiftingError>;
}

impl ParseEventExt for EventData {
    fn parse(self) -> Result<PunEvent, LiftingError> {
        Ok(match self.code {
            // PUN PhotonNetworkPart.cs PhotonNetwork::OnEvent()
            // ordered by the order they are handled in PUN
            pun_event_code::RPC => PunEvent::Rpc(Box::new(RpcEvent::try_from(self.parameters)?)),
            pun_event_code::SEND_SERIALIZE => {
                PunEvent::SendSerialize(Box::new(SendSerializeEvent::try_from(self.parameters)?))
            }
            pun_event_code::SEND_SERIALIZE_RELIABLE => PunEvent::SendSerializeReliable(Box::new(
                SendSerializeEvent::try_from(self.parameters)?,
            )),
            pun_event_code::INSTANTIATION => {
                PunEvent::Instantiation(Box::new(InstantiationEvent::try_from(self.parameters)?))
            }
            pun_event_code::CLOSE_CONNECTION => PunEvent::CloseConnection(Box::new(
                CloseConnectionEvent::try_from(self.parameters)?,
            )),
            pun_event_code::DESTROY_PLAYER => {
                PunEvent::DestroyPlayer(Box::new(DestroyPlayerEvent::try_from(self.parameters)?))
            }
            pun_event_code::DESTROY => {
                PunEvent::Destroy(Box::new(DestroyEvent::try_from(self.parameters)?))
            }
            pun_event_code::OWNERSHIP_REQUEST => PunEvent::OwnershipRequest(Box::new(
                OwnershipRequestEvent::try_from(self.parameters)?,
            )),
            pun_event_code::OWNERSHIP_UPDATE => PunEvent::OwnershipUpdate(Box::new(
                OwnershipUpdateEvent::try_from(self.parameters)?,
            )),

            // PhotonRealtime LoadBalancingClient.cs: LoadBalancingClient::OnEvent()
            event_code::GAME_LIST => {
                PunEvent::GameList(Box::new(RoomInfoList::try_from(self.parameters)?))
            }
            event_code::GAME_LIST_UPDATE => {
                PunEvent::GameListUpdate(Box::new(RoomInfoList::try_from(self.parameters)?))
            }
            event_code::JOIN => PunEvent::Join(Box::new(JoinEvent::try_from(self.parameters)?)),
            event_code::LEAVE => PunEvent::Leave(Box::new(LeaveEvent::try_from(self.parameters)?)),
            event_code::PROPERTIES_CHANGED => PunEvent::PropertiesChanged(Box::new(
                PropertiesChangedEvent::try_from(self.parameters)?,
            )),
            event_code::APP_STATS => {
                PunEvent::AppStats(Box::new(AppStatsEvent::try_from(self.parameters)?))
            }
            event_code::LOBBY_STATS => {
                PunEvent::LobbyStats(Box::new(LobbyStatsEvent::try_from(self.parameters)?))
            }
            event_code::ERROR_INFO => {
                PunEvent::ErrorInfo(Box::new(ErrorInfoEvent::try_from(self.parameters)?))
            }
            event_code::AUTH_EVENT => {
                PunEvent::AuthEvent(Box::new(AuthEvent::try_from(self.parameters)?))
            }

            // TODO: chat event codes?
            _ => {
                return Err(LiftingError::UnknownMessageCode {
                    message_type: PhotonMessageType::EventData,
                    message_code: self.code,
                    parameters: self.parameters,
                    operation_response_data: None,
                });
            }
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PunEvent {
    // orderedby descending event id
    /// Indicates that a player joined the room. This could be the current player.
    Join(Box<JoinEvent>), // 255
    Leave(Box<LeaveEvent>),                         // 254
    PropertiesChanged(Box<PropertiesChangedEvent>), // 253
    // 252 is deprecated `DISCONNECT`
    ErrorInfo(Box<ErrorInfoEvent>), // 251
    // 250 is related to raise event?
    // 249-231: ?
    GameList(Box<RoomInfoList>),       // 230
    GameListUpdate(Box<RoomInfoList>), // 229
    // 228: queue state, unused
    // 227: "match", unused
    /// Sent by the master server in 1 minute intervals
    AppStats(Box<AppStatsEvent>), // 226
    // 225: unknown
    LobbyStats(Box<LobbyStatsEvent>), // 224
    AuthEvent(Box<AuthEvent>),        // 223

    // start pun event codes
    OwnershipUpdate(Box<OwnershipUpdateEvent>), // 212
    // 211: vacant view ids, unused?
    OwnershipTransfer(Box<OwnershipTransferEvent>), // 210
    OwnershipRequest(Box<OwnershipRequestEvent>),   // 209
    // 208: unknown
    DestroyPlayer(Box<DestroyPlayerEvent>),         // 207
    SendSerializeReliable(Box<SendSerializeEvent>), // 206
    // 205: unknown
    Destroy(Box<DestroyEvent>),                 // 204
    CloseConnection(Box<CloseConnectionEvent>), // 203
    Instantiation(Box<InstantiationEvent>),     // 202
    SendSerialize(Box<SendSerializeEvent>),     // 201
    Rpc(Box<RpcEvent>),                         // 200
}

impl PunEvent {
    pub fn unparse(self) -> EventData {
        let (code, parameters) = match self {
            PunEvent::Join(p) => (event_code::JOIN, (*p).into()),
            PunEvent::Leave(p) => (event_code::LEAVE, (*p).into()),
            PunEvent::PropertiesChanged(p) => (event_code::PROPERTIES_CHANGED, (*p).into()),
            PunEvent::ErrorInfo(p) => (event_code::ERROR_INFO, (*p).into()),
            PunEvent::GameList(p) => (event_code::GAME_LIST, (*p).into()),
            PunEvent::GameListUpdate(p) => (event_code::GAME_LIST_UPDATE, (*p).into()),
            PunEvent::AppStats(p) => (event_code::APP_STATS, (*p).into()),
            PunEvent::LobbyStats(p) => (event_code::LOBBY_STATS, (*p).into()),
            PunEvent::AuthEvent(p) => (event_code::AUTH_EVENT, (*p).into()),
            PunEvent::OwnershipUpdate(p) => (pun_event_code::OWNERSHIP_UPDATE, (*p).into()),
            PunEvent::OwnershipTransfer(p) => (pun_event_code::OWNERSHIP_TRANSFER, (*p).into()),
            PunEvent::OwnershipRequest(p) => (pun_event_code::OWNERSHIP_REQUEST, (*p).into()),
            PunEvent::DestroyPlayer(p) => (pun_event_code::DESTROY_PLAYER, (*p).into()),
            PunEvent::SendSerializeReliable(p) => {
                (pun_event_code::SEND_SERIALIZE_RELIABLE, (*p).into())
            }
            PunEvent::Destroy(p) => (pun_event_code::DESTROY, (*p).into()),
            PunEvent::CloseConnection(p) => (pun_event_code::CLOSE_CONNECTION, (*p).into()),
            PunEvent::Instantiation(p) => (pun_event_code::INSTANTIATION, (*p).into()),
            PunEvent::SendSerialize(p) => (pun_event_code::SEND_SERIALIZE, (*p).into()),
            PunEvent::Rpc(p) => (pun_event_code::RPC, (*p).into()),
        };

        EventData { code, parameters }
    }
}

impl_u8_map_conversion! {
    JoinEvent {
        @required
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32,

        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        actor_properties: PhotonHashmap, // TODO: to object

        [parameter_code::ACTOR_LIST => PhotonObject::Array]
        actors_in_room: PhotonArray,
    }

    /// Parameter for [event_code::LEAVE].
    LeaveEvent {
        /// The id of the player who left
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32,

        [parameter_code::ACTOR_LIST => PhotonObject::Array]
        actors: Vec<PhotonObject>,

        /// Only checked if originating player is not null
        [parameter_code::IS_INACTIVE => PhotonObject::Boolean]
        is_inactive: bool,

        /// If not zero, the actor id of the new master client
        [parameter_code::MASTER_CLIENT_ID => PhotonObject::Integer]
        master_client_id: i32,
    }

    /// Parameter for [event_code::PROPERTIES_CHANGED].
    PropertiesChangedEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender: i32, // TODO: present? not used in code

        /// The player whose properties are updated if >=1. If this is 0, indicates properties of the room.
        @required
        [parameter_code::TARGET_ACTOR_NR => PhotonObject::Integer]
        target_actor_number: i32,

        /// If [Self::target_actor_number] is 0, these are game properties. Otherwise, these are actor properties.
        @required
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: PhotonHashmap,
    }

    ErrorInfoEvent {
        [parameter_code::INFO => PhotonObject::String]
        info: String,
    }

    /// Parameter of [event_code::GAME_LIST] and [event_code::GAME_LIST_UPDATE]. Contains a list of [RoomInfo].
    RoomInfoList {
        @required
        [parameter_code::GAME_LIST => PhotonObject::Hashtable]
        games: IndexMap<String, RoomInfo>,
    }

    AppStatsEvent {
        [parameter_code::PEER_COUNT => PhotonObject::Integer]
        peer_count: i32,

        [parameter_code::GAME_COUNT => PhotonObject::Integer]
        room_count: i32,

        [parameter_code::MASTER_PEER_COUNT => PhotonObject::Integer]
        master_peer_count: i32,
    }

    LobbyStatsEvent {
        @required
        [parameter_code::LOBBY_NAME => PhotonObject::StringArray]
        names: Vec<String>,

        [parameter_code::PEER_COUNT => PhotonObject::IntArray]
        peers: Vec<i32>, // props are technically not required, since photon wont crash if they are missing

        [parameter_code::GAME_COUNT => PhotonObject::IntArray]
        rooms: Vec<i32>,

        /// Enum of LobbyType
        [parameter_code::LOBBY_TYPE => PhotonObject::ByteArray]
        lobby_types: Vec<u8>,
    }

    AuthEvent {
        [parameter_code::TOKEN]
        info: PhotonObject,
    }

    OwnershipUpdateEvent {
        @required // not actually used, but should always be present?
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::IntArray]
        request_values: Vec<i32>,
    }

    OwnershipTransferEvent {
        @required
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::IntArray]
        request_values: Vec<i32>,
    }

    OwnershipRequestEvent {
        @required
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::IntArray]
        request_values: Vec<i32>,
    }

    DestroyPlayerEvent {
        [parameter_code::DATA => PhotonObject::Hashtable]
        custom_data: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::SEND_SERIALIZE] and [pun_event_code::SEND_SERIALIZE_RELIABLE].
    SendSerializeEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::DESTROY].
    DestroyEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: DestroyEventData,
    }

    CloseConnectionEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,
    }

    /// Parameter for [pun_event_code::INSTANTIATION].
    InstantiationEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: InstantiationEventData,
    }

    RpcEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required // should never be null according to PUN. Will log error but not crash.
        [parameter_code::CUSTOM_EVENT_CONTENT => PhotonObject::Hashtable]
        data: RpcCall,
    }
}

impl_photon_map_conversion! {
    /// Describes a room.
    #[derive(PartialEq, Eq)]
    RoomInfo {
        /// If `true`, this game should be removed from the game list in the lobby. Not used during gameplay.
        [PhotonObject::Byte(game_property_key::REMOVED) => PhotonObject::Boolean]
        removed: bool,

        /// Indicates how many players can be in this room. 0 means no limit.
        [PhotonObject::Byte(game_property_key::MAX_PLAYERS) => PhotonObject::Byte]
        max_players: u8,

        /// Indicates if the room can be joined.
        [PhotonObject::Byte(game_property_key::IS_OPEN) => PhotonObject::Boolean]
        is_open: bool,

        /// Indicates if this room should be shown in the lobby. Invisible rooms can still be joined.
        [PhotonObject::Byte(game_property_key::IS_VISIBLE) => PhotonObject::Boolean]
        is_visible: bool,

        [PhotonObject::Byte(game_property_key::PLAYER_COUNT) => PhotonObject::Byte]
        player_count: u8,

        [PhotonObject::Byte(game_property_key::CLEANUP_CACHE_ON_LEAVE) => PhotonObject::Boolean]
        cleanup_cache_on_leave: bool,

        /// The actor id of the master client.
        [PhotonObject::Byte(game_property_key::MASTER_CLIENT_ID) => PhotonObject::Integer]
        master_client_id: i32,

        [PhotonObject::Byte(game_property_key::PROPS_LISTED_IN_LOBBY) => PhotonObject::StringArray]
        props_listed_in_lobby: Vec<String>,

        /// Instructs the server to keep player slots open for these players.
        [PhotonObject::Byte(game_property_key::EXPECTED_USERS) => PhotonObject::StringArray]
        expected_users: Vec<String>,

        /// How long the room stays alive after the last player left.
        ///
        /// See also [RoomInfo::player_ttl].
        [PhotonObject::Byte(game_property_key::EMPTY_ROOM_TTL) => PhotonObject::Integer]
        empty_room_ttl: i32,

        /// How long a player stays "active" after disconnecting. As long as this time has not passed, their slot stays occupied.
        [PhotonObject::Byte(game_property_key::PLAYER_TTL) => PhotonObject::Integer]
        player_ttl: i32,
    }

    /// Event data from [DestroyEvent].
    DestroyEventData {
        @required
        [PhotonObject::Byte(0) => PhotonObject::Integer]
        view_id: ViewId,
    }

    /// Event data from [InstantiationEvent].
    InstantiationEventData {
        @required
        [PhotonObject::Byte(0) => PhotonObject::String]
        prefab_name: String,

        /// # Remarks
        /// Of type Vector3
        [PhotonObject::Byte(1) => PhotonObject::Custom]
        position: CustomData,

        /// # Remarks
        /// Of type Quaternion
        [PhotonObject::Byte(2) => PhotonObject::Custom]
        rotation: CustomData,

        [PhotonObject::Byte(3) => PhotonObject::Byte]
        group: u8,

        /// Should be of same length as [Self::incoming_instantiation_data].
        [PhotonObject::Byte(4) => PhotonObject::IntArray]
        views_ids: Vec<i32>, // TODO: should be Vec<ViewId>, but needs more advanced macro

        /// Should be of same length as [Self::views_ids].
        [PhotonObject::Byte(5) => PhotonObject::ObjectArray]
        incoming_instantiation_data: PhotonObjectArray,

        @required
        [PhotonObject::Byte(6) => PhotonObject::Integer]
        server_time: i32,

        /// The view id
        @required
        [PhotonObject::Byte(7) => PhotonObject::Integer]
        instantiation_id: ViewId,

        [PhotonObject::Byte(8) => PhotonObject::Short]
        obj_level_prefix: i16,
    }

    /// An RPC call. Can be both sent and received by the client.
    RpcCall {
        @required
        [PhotonObject::Byte(0) => PhotonObject::Integer]
        net_view_id: ViewId,

        [PhotonObject::Byte(1) => PhotonObject::Short]
        other_side_prefix: i16,

        /// Present when sent from client to server, but not the other way around
        [PhotonObject::Byte(2) => PhotonObject::Integer]
        server_timestamp: i32,

        /// Mutually exclusive with [RpcCall::rpc_index]
        [PhotonObject::Byte(3) => PhotonObject::String]
        method_name: String,

        [PhotonObject::Byte(4) => PhotonObject::ObjectArray]
        in_method_parameters: Vec<PhotonObject>,

        /// Mutually exclusive with [RpcCall::method_name]
        [PhotonObject::Byte(5) => PhotonObject::Byte]
        rpc_index: u8,
    }
}

/// A serialized object stream. Can represent a `Monobehavior`, a `Transform`, a `Rigidbody` or a `RigidBody2D`.
///
/// See [SendSerializeEvent].
pub struct SerializedData {
    pub view_id: i32,
    pub data_stream: Vec<PhotonObject>,
}

impl SendSerializeEvent {
    // TODO: proper error type. probably want something generic like InvalidDataError
    // Gets a copy of the serialized data in this event
    pub fn get_serialized_data(&self) -> Option<Vec<SerializedData>> {
        Self::parse_serialized_data(&self.data)
    }

    pub fn parse_serialized_data(data: &PhotonHashmap) -> Option<Vec<SerializedData>> {
        _ = data.0.get(&PhotonObject::Byte(1))?;

        let header_len = match data.0.contains_key(&PhotonObject::Byte(1)) {
            true => 2,
            false => 1,
        };

        const DATA_INITIAL_INDEX: usize = 10;
        let data_len = data.0.len() - header_len;
        let mut ret = vec![];
        for i in 0..data_len {
            // items start at key 10 and count up
            // the official implementation has various opportunities to crash here, but we should be safe
            let index = i + DATA_INITIAL_INDEX;
            let index = (index & 0xFF) as u8; // NOTE: official implementation does wrap here

            let found = data.0.get(&PhotonObject::Byte(index));
            let found = match found {
                Some(PhotonObject::ObjectArray(PhotonObjectArray(x))) => x,
                _ => return None,
            };
            let data = SerializedData::from_object_array(found.clone())?;
            ret.push(data);
        }

        Some(ret)
    }
}

impl SerializedData {
    // TODO: views can have specific synchronisation. User should pass that to this method
    // TODO: better errors
    pub fn from_object_array(mut data: Vec<PhotonObject>) -> Option<Self> {
        if data.len() < 3 {
            return None;
        }

        let view_id = match data[0] {
            PhotonObject::Integer(i) => i,
            _ => return None,
        };
        // index 1 and 2 are related to compression, which is not implemented yet
        let data_stream = data.drain(3..).collect();

        Some(Self {
            view_id,
            data_stream,
        })
    }

    pub fn get_view_id(&self) -> ViewId {
        ViewId(self.view_id)
    }

    // NOTE: could add an implementation to parse this component as a Transform, Rigidbody or Rigidbody2D, as they have
    // pre-defined data streams. The user would have to pass their `onSerializeTransformOption` or
    // `onSerializeRigidBodyOption` value, though.
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;
    use ordered_float::OrderedFloat;

    use super::RoomInfo;
    use crate::photon::object::PhotonObject;
    use crate::pun::constants::game_property_key;

    #[test]
    fn room_info() {
        let room_info = RoomInfo {
            removed: None,
            max_players: Some(15),
            is_open: Some(true),
            is_visible: None,
            player_count: Some(3),
            cleanup_cache_on_leave: None,
            master_client_id: None,
            props_listed_in_lobby: None,
            expected_users: None,
            empty_room_ttl: None,
            player_ttl: None,
            custom_properties: indexmap! {
                "switchingmap".into() => PhotonObject::Boolean(false),
                "meanKD".into() => PhotonObject::Float(OrderedFloat(0.72795415)),
                "seasonID".into() => PhotonObject::String("".into()),
                "eventcode".into() => PhotonObject::Integer(0)
            },
        };

        let photon_map = crate::PhotonHashmap(indexmap! {
            PhotonObject::String("switchingmap".into()) => PhotonObject::Boolean(false),
            PhotonObject::Byte(game_property_key::MAX_PLAYERS) => PhotonObject::Byte(15),
            PhotonObject::String("meanKD".into()) => PhotonObject::Float(OrderedFloat(0.72795415)),
            PhotonObject::Byte(game_property_key::IS_OPEN) => PhotonObject::Boolean(true),
            PhotonObject::String("seasonID".into()) => PhotonObject::String("".into()),
            PhotonObject::Byte(game_property_key::PLAYER_COUNT) => PhotonObject::Byte(3),
            PhotonObject::String("eventcode".into()) => PhotonObject::Integer(0)
        });

        {
            let deserialized: crate::PhotonHashmap = room_info.clone().into();
            assert_eq!(deserialized, photon_map);
        }

        {
            let serialized = RoomInfo::try_from(photon_map).unwrap();
            assert_eq!(serialized, room_info);
        }
    }
}
