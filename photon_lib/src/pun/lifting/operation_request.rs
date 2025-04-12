use serde::Serialize;

use crate::{
    ParameterMap, PhotonArray, PhotonHashmap,
    photon::{
        message::{EventData, OperationRequest, PhotonMessageType},
        object::PhotonObject,
    },
    pun::{LiftingError, constants::*},
};

use super::{ParseEventExt, PunEvent};

pub trait ParseOperationRequestExt {
    fn parse(self) -> Result<PunOperationRequest, LiftingError>;
}

impl ParseOperationRequestExt for OperationRequest {
    fn parse(self) -> Result<PunOperationRequest, LiftingError> {
        Ok(match self.operation_code {
            // PhotonRealtime LoadBalancingClient.cs: LoadBalancingClient::OpWebRpc()
            operation_code::WEB_RPC => {
                PunOperationRequest::WebRpc(Box::new(WebRpcRequest::try_from(self.parameters)?))
            }

            // PhotonRealtime LoadBalancingPeer.cs: LoadBalancingPeer::Op*()
            operation_code::GET_REGIONS => PunOperationRequest::GetRegions(Box::new(
                GetRegionsRequest::try_from(self.parameters)?,
            )),
            operation_code::JOIN_LOBBY => PunOperationRequest::JoinLobby(Box::new(
                JoinLobbyRequest::try_from(self.parameters)?,
            )),
            operation_code::LEAVE_LOBBY => PunOperationRequest::LeaveLobby(Box::new(
                LeaveLobbyRequest::try_from(self.parameters)?,
            )),
            operation_code::CREATE_GAME => PunOperationRequest::CreateGame(Box::new(
                CreateGameRequest::try_from(self.parameters)?,
            )),
            operation_code::JOIN_GAME => {
                PunOperationRequest::JoinGame(Box::new(JoinGameRequest::try_from(self.parameters)?))
            }
            operation_code::JOIN_RANDOM_GAME => PunOperationRequest::JoinRandomGame(Box::new(
                JoinRandomGameRequest::try_from(self.parameters)?,
            )),
            operation_code::LEAVE => {
                PunOperationRequest::Leave(Box::new(LeaveRequest::try_from(self.parameters)?))
            }
            operation_code::GET_GAME_LIST => PunOperationRequest::GetGameList(Box::new(
                GetGameListRequest::try_from(self.parameters)?,
            )),
            operation_code::FIND_FRIENDS => PunOperationRequest::FindFriends(Box::new(
                FindFriendsRequest::try_from(self.parameters)?,
            )),
            operation_code::SET_PROPERTIES => PunOperationRequest::SetProperties(Box::new(
                SetPropertiesRequest::try_from(self.parameters)?,
            )),
            operation_code::AUTHENTICATE => PunOperationRequest::Authenticate(Box::new(
                AuthenticateRequest::try_from(self.parameters)?,
            )),
            operation_code::AUTHENTICATE_ONCE => PunOperationRequest::AuthenticateOnce(Box::new(
                AuthenticateOnceRequest::try_from(self.parameters)?,
            )),
            operation_code::CHANGE_GROUPS => PunOperationRequest::ChangeGroups(Box::new(
                ChangeGroupsRequest::try_from(self.parameters)?,
            )),
            operation_code::RAISE_EVENT => PunOperationRequest::RaiseEvent(Box::new(
                RaiseEventRequest::try_from(self.parameters)?,
            )),
            operation_code::SERVER_SETTINGS => PunOperationRequest::ServerSettings(Box::new(
                ServerSettingsRequest::try_from(self.parameters)?,
            )),

            _ => {
                return Err(LiftingError::UnknownMessageCode {
                    message_type: PhotonMessageType::OperationRequest,
                    message_code: self.operation_code,
                    parameters: self.parameters,
                    operation_response_data: None,
                });
            }
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum PunOperationRequest {
    // 255: join?
    Leave(Box<LeaveRequest>),                 // 254
    RaiseEvent(Box<RaiseEventRequest>),       // 253
    SetProperties(Box<SetPropertiesRequest>), // 252
    // 251: GetProperties
    ChangeGroups(Box<ChangeGroupsRequest>),         // 248
    AuthenticateOnce(Box<AuthenticateOnceRequest>), // 231
    Authenticate(Box<AuthenticateRequest>),         // 230
    JoinLobby(Box<JoinLobbyRequest>),               // 229
    LeaveLobby(Box<LeaveLobbyRequest>),             // 228
    CreateGame(Box<CreateGameRequest>),             // 227
    JoinGame(Box<JoinGameRequest>),                 // 226
    JoinRandomGame(Box<JoinRandomGameRequest>),     // 225
    FindFriends(Box<FindFriendsRequest>),           // 222
    // 221: GetLobbyStats
    GetRegions(Box<GetRegionsRequest>),         // 220
    WebRpc(Box<WebRpcRequest>),                 // 219
    ServerSettings(Box<ServerSettingsRequest>), // 218
    GetGameList(Box<GetGameListRequest>),       // 217
}

impl PunOperationRequest {
    pub fn unparse(self) -> OperationRequest {
        let (operation_code, parameters) = match self {
            PunOperationRequest::Leave(p) => (operation_code::LEAVE, (*p).into()),
            PunOperationRequest::RaiseEvent(p) => (operation_code::RAISE_EVENT, (*p).into()),
            PunOperationRequest::SetProperties(p) => (operation_code::SET_PROPERTIES, (*p).into()),
            PunOperationRequest::ChangeGroups(p) => (operation_code::CHANGE_GROUPS, (*p).into()),
            PunOperationRequest::AuthenticateOnce(p) => {
                (operation_code::AUTHENTICATE_ONCE, (*p).into())
            }
            PunOperationRequest::Authenticate(p) => (operation_code::AUTHENTICATE, (*p).into()),
            PunOperationRequest::JoinLobby(p) => (operation_code::JOIN_LOBBY, (*p).into()),
            PunOperationRequest::LeaveLobby(p) => (operation_code::LEAVE_LOBBY, (*p).into()),
            PunOperationRequest::CreateGame(p) => (operation_code::CREATE_GAME, (*p).into()),
            PunOperationRequest::JoinGame(p) => (operation_code::JOIN_GAME, (*p).into()),
            PunOperationRequest::JoinRandomGame(p) => {
                (operation_code::JOIN_RANDOM_GAME, (*p).into())
            }
            PunOperationRequest::FindFriends(p) => (operation_code::FIND_FRIENDS, (*p).into()),
            PunOperationRequest::GetRegions(p) => (operation_code::GET_REGIONS, (*p).into()),
            PunOperationRequest::WebRpc(p) => (operation_code::WEB_RPC, (*p).into()),
            PunOperationRequest::ServerSettings(p) => {
                (operation_code::SERVER_SETTINGS, (*p).into())
            }
            PunOperationRequest::GetGameList(p) => (operation_code::GET_GAME_LIST, (*p).into()),
        };

        OperationRequest {
            operation_code,
            parameters,
        }
    }
}

impl_u8_map_conversion! {
    LeaveRequest {
        /// `None` implies `false`. This is never explicitly set to `false`.
        [parameter_code::IS_INACTIVE => PhotonObject::Boolean]
        is_inactive: bool,

        /// Set to either `None` or `0x02` (`WebFlags.SendAuthCookieConst`).
        [parameter_code::EVENT_FORWARD => PhotonObject::Byte]
        event_forward: u8,
    }

    RaiseEventRequest {
        /// Identifies this type of event (and the content). The game's event codes can start with 0.
        @required
        [parameter_code::CODE => PhotonObject::Byte]
        code: u8,

        /// `EventCaching` setting. Defines if the server should simply send the event, put it in the cache or remove
        /// events that are like this one.
        ///
        /// If this is not `None`, there is no data field.
        [parameter_code::CACHE => PhotonObject::Byte]
        cache: u8,

        /// Any serializable datatype (including Hashtable like the other OpRaiseEvent overloads).
        [parameter_code::DATA]
        data: PhotonObject,

        [parameter_code::ACTOR_LIST => PhotonObject::Array]
        actor_list: PhotonArray, // ints

        [parameter_code::GROUP => PhotonObject::Byte]
        group: u8,

        [parameter_code::RECEIVER_GROUP => PhotonObject::Byte]
        receiver_group: u8,

        [parameter_code::EVENT_FORWARD => PhotonObject::Byte]
        event_forward: u8,
    }

    SetPropertiesRequest {
        @required
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: PhotonHashmap,

        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32, // ActorNr, if `None` then target is room

        [parameter_code::BROADCAST => PhotonObject::Boolean]
        broadcast: bool, // always present?

        // [parameter_code::EXPECTED_VALUES => PhotonObject::Hashtable]
        // expected_values: PhotonHashmap, // not in code

        /// Only if HttpForward is set?
        [parameter_code::EVENT_FORWARD => PhotonObject::Byte]
        event_forward: u8,
    }

    ChangeGroupsRequest {
        [parameter_code::REMOVE => PhotonObject::ByteArray]
        remove: Vec<u8>,

        [parameter_code::ADD => PhotonObject::ByteArray]
        add: Vec<u8>,
    }

    AuthenticateOnceRequest {
        [parameter_code::EXPECTED_PROTOCOL => PhotonObject::Byte]
        expected_protocol: u8,

        [parameter_code::ENCRYPTION_MODE => PhotonObject::Byte]
        expected_mode: u8,

        [parameter_code::APP_VERSION => PhotonObject::String]
        app_version: String,

        [parameter_code::APPLICATION_ID => PhotonObject::String]
        application_id: String,

        [parameter_code::REGION => PhotonObject::String]
        region: String,

        [parameter_code::USER_ID => PhotonObject::String]
        user_id: String,

        [parameter_code::TOKEN]
        token: PhotonObject,

        [parameter_code::CLIENT_AUTHENTICATION_TYPE => PhotonObject::Byte]
        client_authentication_type: u8,

        [parameter_code::CLIENT_AUTHENTICATION_PARAMS => PhotonObject::String]
        client_authentication_params: String,

        [parameter_code::CLIENT_AUTHENTICATION_DATA]
        client_post_data: PhotonObject,
    }

    #[derive(Default)]
    AuthenticateRequest {
        /// If true, request lobby stats
        [parameter_code::LOBBY_STATS => PhotonObject::Boolean]
        lobby_stats: bool,

        [parameter_code::APP_VERSION => PhotonObject::String]
        app_version: String,

        [parameter_code::APPLICATION_ID => PhotonObject::String]
        application_id: String,

        [parameter_code::REGION => PhotonObject::String]
        region: String,

        [parameter_code::USER_ID => PhotonObject::String]
        user_id: String,

        [parameter_code::TOKEN]
        token: PhotonObject,

        [parameter_code::CLIENT_AUTHENTICATION_TYPE => PhotonObject::Byte]
        client_authentication_type: u8,

        [parameter_code::CLIENT_AUTHENTICATION_PARAMS => PhotonObject::String]
        client_authentication_params: String,

        [parameter_code::CLIENT_AUTHENTICATION_DATA]
        client_post_data: PhotonObject,
    }

    JoinLobbyRequest {
        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,
    }

    LeaveLobbyRequest {
        // no properties, parameters is `null`
    }

    CreateGameRequest {
        [parameter_code::ROOM_NAME => PhotonObject::String]
        room_name: String,

        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,

        [parameter_code::ADD => PhotonObject::StringArray]
        add: Vec<String>, // if this is set, the packet is encrypted?

        [parameter_code::TICKET]
        ticket: PhotonObject,

        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        player_properties: PhotonHashmap,

        /// Broadcast actor properties
        [parameter_code::BROADCAST => PhotonObject::Boolean]
        broadcast: bool,

        // room options

        /// Mutually exclusive with [Self::properties]. This is the default.
        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        game_properties: GameProperties,

        /// Mutually exclusive with [Self::game_properties]. Likely never used.
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: GameProperties,

        [parameter_code::CLEANUP_CACHE_ON_LEAVE => PhotonObject::Boolean]
        cleanup_cache_on_leave: bool,

        [parameter_code::CHECK_USER_ON_JOIN => PhotonObject::Boolean]
        check_user_on_join: bool,

        [parameter_code::PLAYER_TTL => PhotonObject::Integer]
        player_ttl: i32,

        [parameter_code::EMPTY_ROOM_TTL => PhotonObject::Integer]
        empty_room_ttl: i32,

        [parameter_code::SUPPRESS_ROOM_EVENTS => PhotonObject::Boolean]
        suppress_room_events: bool,

        [parameter_code::PLUGINS => PhotonObject::Array]
        plugins: PhotonArray,

        [parameter_code::PUBLISH_USER_ID => PhotonObject::Boolean]
        publish_user_id: bool,

        [parameter_code::ROOM_OPTION_FLAGS => PhotonObject::Integer]
        flags: i32,
    }

    JoinGameRequest {
        [parameter_code::ROOM_NAME => PhotonObject::String]
        room_name: String,

        [parameter_code::JOIN_MODE => PhotonObject::Byte]
        join_mode: u8,

        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,

        [parameter_code::ADD => PhotonObject::StringArray]
        add: Vec<String>, // if this is set, the packet is encrypted?

        [parameter_code::TICKET]
        ticket: PhotonObject,

        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        player_properties: PhotonHashmap,

        /// Broadcast actor properties
        [parameter_code::BROADCAST => PhotonObject::Boolean]
        broadcast: bool,

        // room options

        /// Mutually exclusive with [Self::properties]. This is the default.
        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        game_properties: GameProperties,

        /// Mutually exclusive with [Self::game_properties]. Likely never used.
        [parameter_code::PROPERTIES => PhotonObject::Hashtable]
        properties: GameProperties,

        [parameter_code::CLEANUP_CACHE_ON_LEAVE => PhotonObject::Boolean]
        cleanup_cache_on_leave: bool,

        [parameter_code::CHECK_USER_ON_JOIN => PhotonObject::Boolean]
        check_user_on_join: bool,

        [parameter_code::PLAYER_TTL => PhotonObject::Integer]
        player_ttl: i32,

        [parameter_code::EMPTY_ROOM_TTL => PhotonObject::Integer]
        empty_room_ttl: i32,

        [parameter_code::SUPPRESS_ROOM_EVENTS => PhotonObject::Boolean]
        suppress_room_events: bool,

        [parameter_code::PLUGINS => PhotonObject::StringArray]
        plugins: Vec<String>,

        [parameter_code::PUBLISH_USER_ID => PhotonObject::Boolean]
        publish_user_id: bool,

        [parameter_code::ROOM_OPTION_FLAGS => PhotonObject::Integer]
        flags: i32,
    }

    JoinRandomGameRequest {
        [game_property_key::MAX_PLAYERS => PhotonObject::Byte]
        max_players: u8,

        [game_property_key::MAX_PLAYERS_INT => PhotonObject::Integer]
        max_players_int: i32,

        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        expected_room_properties: PhotonHashmap,

        [parameter_code::MATCH_MAKING_TYPE => PhotonObject::Byte]
        match_making_type: u8,

        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,

        /// If present, not null or empty
        [parameter_code::DATA => PhotonObject::String]
        sql_lobby_filter: String,

        [parameter_code::ADD => PhotonObject::StringArray]
        add: Vec<String>, // if this is set, the packet is encrypted?

        [parameter_code::TICKET]
        ticket: PhotonObject,

        [parameter_code::ALLOW_REPEATS => PhotonObject::Boolean]
        allow_repeats: bool, // Always present and true?
    }

    FindFriendsRequest {
        [parameter_code::FIND_FRIENDS_REQUEST_LIST => PhotonObject::StringArray]
        friends_to_find: Vec<String>,

        [parameter_code::FIND_FRIENDS_OPTIONS => PhotonObject::Integer]
        options: i32,
    }

    GetRegionsRequest {
        [parameter_code::APPLICATION_ID => PhotonObject::String]
        app_id: String,
    }

    WebRpcRequest {
        @required
        [parameter_code::URI_PATH => PhotonObject::String]
        uri_path: String,

        [parameter_code::WEB_RPC_PARAMETERS]
        parameters: PhotonObject,

        [parameter_code::EVENT_FORWARD => PhotonObject::Byte]
        event_forward: u8,
    }

    ServerSettingsRequest {
        [0 => PhotonObject::Boolean]
        receive_lobby_stats: bool,
    }

    GetGameListRequest {
        [parameter_code::LOBBY_NAME => PhotonObject::String]
        lobby_name: String,

        [parameter_code::LOBBY_TYPE => PhotonObject::Byte]
        lobby_type: u8,

        [parameter_code::DATA => PhotonObject::String]
        query_data: String,
    }
}

impl_photon_map_conversion! {
    GameProperties {
        @required
        [PhotonObject::Byte(game_property_key::IS_OPEN) => PhotonObject::Boolean]
        is_open: bool,

        @required
        [PhotonObject::Byte(game_property_key::IS_VISIBLE) => PhotonObject::Boolean]
        is_visible: bool,

        @required
        [PhotonObject::Byte(game_property_key::PROPS_LISTED_IN_LOBBY) => PhotonObject::Array]
        props_listed_in_lobby: PhotonArray,

        [PhotonObject::Byte(game_property_key::MAX_PLAYERS) => PhotonObject::Byte]
        max_players: u8,

        [PhotonObject::Byte(game_property_key::MAX_PLAYERS_INT) => PhotonObject::Integer]
        max_players_int: i32,

        [PhotonObject::Byte(game_property_key::CLEANUP_CACHE_ON_LEAVE) => PhotonObject::Boolean]
        cleanup_cache_on_leave: bool,
    }
}

#[derive(Serialize)]
pub struct RaiseEventParsed {
    /// `EventCaching` setting. Defines if the server should simply send the event, put it in the cache or remove
    /// events that are like this one.
    ///
    /// If this is not `None`, there is no data field.
    pub cache: Option<u8>,

    /// Any serializable datatype (including Hashtable like the other OpRaiseEvent overloads).
    pub data: PunEvent,

    pub actor_list: Option<PhotonArray>, // ints

    pub group: Option<u8>,

    pub receiver_group: Option<u8>,

    pub event_forward: Option<u8>,
}

impl TryFrom<RaiseEventRequest> for RaiseEventParsed {
    type Error = LiftingError;

    fn try_from(value: RaiseEventRequest) -> Result<Self, Self::Error> {
        let mut parameters = ParameterMap::default();
        if let Some(data) = value.data {
            parameters
                .0
                .insert(parameter_code::CUSTOM_EVENT_CONTENT, data);
        }
        parameters
            .0
            .insert(parameter_code::ACTOR_NR, PhotonObject::Integer(0));

        let event_data = EventData {
            code: value.code,
            parameters,
        };

        let pun_event = event_data.parse()?;

        Ok(Self {
            cache: value.cache,
            data: pun_event,
            actor_list: value.actor_list,
            group: value.group,
            receiver_group: value.receiver_group,
            event_forward: value.event_forward,
        })
    }
}
