//! Code and types for lifting Photon messages to PUN types

mod events;
mod internal_operation_request;
mod internal_operation_response;
mod operation_request;
mod operation_response;

pub use events::*;
pub use internal_operation_request::*;
pub use internal_operation_response::*;
pub use operation_request::*;
pub use operation_response::*;

use crate::{PhotonArray, PhotonObject};

use super::constants::{actor_properties, game_property_key};

impl_u8_map_conversion! {
    EmptyResponse { }
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

        [PhotonObject::Byte(game_property_key::PROPS_LISTED_IN_LOBBY) => PhotonObject::Array]
        props_listed_in_lobby: PhotonArray,

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

    /// Describes an actor
    ActorInfo {
        @required
        [PhotonObject::Byte(actor_properties::PLAYER_NAME) => PhotonObject::String]
        player_name: String,

        // TODO: IS_INACTIVE and USER_ID
    }
}
