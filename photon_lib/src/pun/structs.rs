//! High-level representations of photon messages.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::PhotonHashmap;
use crate::photon::object::{CustomData, PhotonObject};
use crate::pun::constants::{actor_properties, game_property_key, parameter_code};
#[cfg(doc)]
use crate::pun::constants::{event_code, operation_code, pun_event_code};

/// Represents a Photon View ID
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewId(pub i32);

// NOTE: be very cautious when applying `@required`, parsing will fail if it is not present!
// if you cannot prove that a property is actually always present, do not apply it.
// Basically, always be more cautious than PUN is.

impl_u8_map_conversion! {
    /// Parameter of [event_code::GAME_LIST] and [event_code::GAME_LIST_UPDATE]. Contains a list of [RoomInfo].
    #[derive(Debug)]
    RoomInfoList {
        @required
        [parameter_code::GAME_LIST => PhotonObject::Hashtable]
        games: IndexMap<String, RoomInfo>,
    }

    SetPropertiesOperationRequest {
        @required
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: PhotonHashmap,

        /// Only present when updating an actor, not when updating a room.
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32,

        @required
        [parameter_code::BROADCAST => PhotonObject::Boolean]
        broadcast: bool,

        [parameter_code::EXPECTED_VALUES => PhotonObject::Hashtable]
        expected_values: PhotonHashmap,

        [parameter_code::EVENT_FORWARD => PhotonObject::Boolean]
        event_forward: bool,
    }

    /// Request parameter of [operation_code::JOIN_GAME].
    #[derive(Debug)]
    JoinGameRequest {
        [parameter_code::ROOM_NAME => PhotonObject::String]
        room_name: String,

        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: PhotonHashmap,

        [parameter_code::BROADCAST => PhotonObject::Boolean]
        broadcast: bool,

        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        player_properties: PhotonHashmap,

        /// A serialized instance of [RoomInfo]
        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        game_properties: PhotonHashmap,

        [parameter_code::CLEANUP_CACHE_ON_LEAVE => PhotonObject::Boolean]
        cleanup_cache_on_leave: bool,

        [parameter_code::PUBLISH_USER_ID => PhotonObject::Boolean]
        publis_user_id: bool,

        [parameter_code::ADD => PhotonObject::StringArray]
        add: Vec<String>,

        [parameter_code::SUPPRESS_ROOM_EVENTS => PhotonObject::Boolean]
        suppress_room_events: bool,

        [parameter_code::EMPTY_ROOM_TTL => PhotonObject::Integer]
        empty_room_ttl: i32,

        [parameter_code::PLAYER_TTL => PhotonObject::Integer]
        player_ttl: i32,

        [parameter_code::CHECK_USER_ON_JOIN => PhotonObject::Boolean]
        check_user_on_join: bool,

        [parameter_code::JOIN_MODE => PhotonObject::Byte]
        join_mode: u8,

        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,

        [parameter_code::PLUGINS => PhotonObject::StringArray]
        plugins: Vec<String>,

        [parameter_code::ROOM_OPTION_FLAGS => PhotonObject::Integer]
        room_option_flags: i32, // could add an impl to map this to an enum or something
    }

    /// Response parameter of [operation_code::JOIN_GAME] on success (return code 0).
    #[derive(Debug)]
    JoinGameResponseSuccess {
        [parameter_code::ROOM_NAME => PhotonObject::String]
        room_name: String,

        @required
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32,

        [parameter_code::ACTOR_LIST => PhotonObject::IntArray]
        actor_list: Vec<i32>,

        /// A hashmap over serialized [Player]s. The keys in this hashmap are integer actor ids.
        @required
        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        player_properties: PhotonHashmap,

        /// A serialized instance of [RoomInfo]
        @required
        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        game_properties: PhotonHashmap,

        [parameter_code::ADDRESS => PhotonObject::String]
        address: String,

        [parameter_code::ROOM_OPTION_FLAGS => PhotonObject::Integer]
        room_option_flags: i32, // could add an impl to map this to an enum or something
    }

    /// Request parameter of [operation_code::RAISE_EVENT]
    #[derive(Debug)]
    RaiseEvent {
        @required
        [parameter_code::CODE => PhotonObject::Byte]
        event_code: u8,

        [parameter_code::DATA]
        data: PhotonObject,

        [parameter_code::CACHE => PhotonObject::Byte]
        cache: u8,

        [parameter_code::RECEIVER_GROUP => PhotonObject::Byte]
        receiver_group: u8,

        [parameter_code::GROUP => PhotonObject::Byte]
        interest_group: u8,

        [parameter_code::ACTOR_LIST => PhotonObject::IntArray]
        actor_list: Vec<i32>,

        [parameter_code::EVENT_FORWARD => PhotonObject::Boolean]
        event_forward: bool,
    }

    /// Parameter for [event_code::LEAVE].
    #[derive(Debug)]
    LeaveEvent {
        /// The id of the player who left
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        [parameter_code::ACTOR_LIST => PhotonObject::Array]
        actors: Vec<PhotonObject>,

        [parameter_code::IS_INACTIVE => PhotonObject::Boolean]
        is_inactive: bool,

        /// The new master client
        [parameter_code::MASTER_CLIENT_ID => PhotonObject::Integer]
        master_client_id: i32,
    }

    /// Parameter for [event_code::PROPERTIES_CHANGED].
    #[derive(Debug)]
    PropertiesChangedEvent {
        /// The id of the player who left
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::TARGET_ACTOR_NR => PhotonObject::Integer]
        target_actor_number: i32,

        /// If [Self::target_actor_number] is 0, these are game properties. Otherwise these are actor properties.
        @required
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::DESTROY].
    DestroyEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::INSTANTIATION].
    InstantiationEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::SEND_SERIALIZE] and [pun_event_code::SEND_SERIALIZE_RELIABLE].
    SendSerializeEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::DATA => PhotonObject::Hashtable]
        data: PhotonHashmap,
    }

    /// Parameter for [pun_event_code::RPC]. Contains a single [RpcCall].
    #[derive(Debug)]
    RpcEvent {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        sender_actor: i32,

        @required
        [parameter_code::CUSTOM_EVENT_CONTENT => PhotonObject::Hashtable]
        data: PhotonHashmap,
    }
}

// NOTE: this macro adds a `custom_properties` field for remaining, string-keyed properties
impl_photon_map_conversion! {
    /// Describes a room.
    #[derive(Debug, Clone, PartialEq, Eq)]
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
        ///
        /// See also [Player::is_inactive].
        [PhotonObject::Byte(game_property_key::PLAYER_TTL) => PhotonObject::Integer]
        player_ttl: i32,
    }

    /// Describes a player. Most information wil be in [Player::custom_properties].
    #[derive(Debug)]
    Player {
        // It's possible that this is always present, but I'm not 100% sure.
        /// The player's nickname.
        [PhotonObject::Byte(actor_properties::PLAYER_NAME) => PhotonObject::String]
        nickname: String,

        // Surprisingly, not always present.
        [PhotonObject::Byte(actor_properties::USER_ID) => PhotonObject::String]
        user_id: String,

        [PhotonObject::Byte(actor_properties::IS_INACTIVE) => PhotonObject::Boolean]
        is_inactive: bool,
    }

    /// Event data from [DestroyEvent].
    #[derive(Debug)]
    DestroyEventData {
        @required
        [PhotonObject::Byte(0) => PhotonObject::Integer]
        view_id: i32,
    }

    /// Event data from [InstantiationEvent].
    #[derive(Debug)]
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
        views_ids: Vec<i32>,

        /// Should be of same length as [Self::views_ids].
        [PhotonObject::Byte(5) => PhotonObject::ObjectArray]
        incoming_instantiation_data: Vec<PhotonObject>,

        @required
        [PhotonObject::Byte(6) => PhotonObject::Integer]
        server_time: i32,

        /// The view id
        @required
        [PhotonObject::Byte(7) => PhotonObject::Integer]
        instantiation_id: i32,

        [PhotonObject::Byte(8) => PhotonObject::Short]
        obj_level_prefix: i16,
    }

    /// An RPC call. Can be both sent and received by the client.
    RpcCall {
        @required
        [PhotonObject::Byte(0) => PhotonObject::Integer]
        net_view_id: i32,

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
