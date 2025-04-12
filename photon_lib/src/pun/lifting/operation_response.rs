use crate::{
    PhotonArray, PhotonHashmap,
    photon::{
        message::{OperationResponse, PhotonMessageType},
        object::PhotonObject,
    },
    pun::{LiftingError, constants::*},
};

use super::EmptyResponse;

pub trait ParseOperationResponseExt {
    fn parse(self) -> Result<(PunOperationResponse, i16, Option<String>), LiftingError>;
}

impl ParseOperationResponseExt for OperationResponse {
    fn parse(self) -> Result<(PunOperationResponse, i16, Option<String>), LiftingError> {
        Ok((
            match self.operation_code {
                // PUN PhotonNetworkPart.cs PhotonNetwork::OnOperation() contains GET_REGIONS, JOIN_GAME, but no parsing

                // PhotonRealtime LoadBalancingClient.cs: LoadBalancingClient::OnOperationResponse()
                operation_code::AUTHENTICATE => PunOperationResponse::Authenticate(Box::new(
                    AuthenticateResponse::try_from(self.parameters)?,
                )),
                operation_code::AUTHENTICATE_ONCE => PunOperationResponse::AuthenticateOnce(
                    Box::new(AuthenticateResponse::try_from(self.parameters)?),
                ),
                operation_code::GET_REGIONS => PunOperationResponse::GetRegions(Box::new(
                    GetRegionsResponse::try_from(self.parameters)?,
                )),
                operation_code::JOIN_RANDOM_GAME => PunOperationResponse::JoinRandomGame(Box::new(
                    JoinGameResponse::try_from(self.parameters)?,
                )),
                operation_code::CREATE_GAME => PunOperationResponse::CreateGame(Box::new(
                    JoinGameResponse::try_from(self.parameters)?,
                )),
                operation_code::JOIN_GAME => PunOperationResponse::JoinGame(Box::new(
                    JoinGameResponse::try_from(self.parameters)?,
                )),
                operation_code::GET_GAME_LIST => PunOperationResponse::GetGameList(Box::new(
                    GetGameListResponse::try_from(self.parameters)?,
                )),
                operation_code::JOIN_LOBBY => PunOperationResponse::JoinLobby(Box::new(
                    EmptyResponse::try_from(self.parameters)?,
                )),
                operation_code::LEAVE_LOBBY => PunOperationResponse::LeaveLobby(Box::new(
                    EmptyResponse::try_from(self.parameters)?,
                )),
                operation_code::LEAVE => {
                    PunOperationResponse::Leave(Box::new(EmptyResponse::try_from(self.parameters)?))
                }
                operation_code::FIND_FRIENDS => PunOperationResponse::FindFriends(Box::new(
                    FindFriendsResponse::try_from(self.parameters)?,
                )),
                operation_code::WEB_RPC => PunOperationResponse::WebRpc(Box::new(
                    EmptyResponse::try_from(self.parameters)?,
                )),

                // other, not specifically handled
                operation_code::RAISE_EVENT => PunOperationResponse::RaiseEvent(Box::new(
                    EmptyResponse::try_from(self.parameters)?,
                )),
                operation_code::SET_PROPERTIES => PunOperationResponse::SetProperties(Box::new(
                    EmptyResponse::try_from(self.parameters)?,
                )),

                _ => {
                    return Err(LiftingError::UnknownMessageCode {
                        message_type: PhotonMessageType::OperationResponse,
                        message_code: self.operation_code,
                        parameters: self.parameters,
                        operation_response_data: Some((self.return_code, self.debug_message)),
                    });
                }
            },
            self.return_code,
            self.debug_message,
        ))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PunOperationResponse {
    Leave(Box<EmptyResponse>),                   // 253
    RaiseEvent(Box<EmptyResponse>),              // 253
    SetProperties(Box<EmptyResponse>),           // 252
    AuthenticateOnce(Box<AuthenticateResponse>), // 231
    Authenticate(Box<AuthenticateResponse>),     // 230
    JoinLobby(Box<EmptyResponse>),               // 229
    LeaveLobby(Box<EmptyResponse>),              // 228
    CreateGame(Box<JoinGameResponse>),           // 227
    JoinGame(Box<JoinGameResponse>),             // 226
    JoinRandomGame(Box<JoinGameResponse>),       // 225
    FindFriends(Box<FindFriendsResponse>),       // 222
    GetRegions(Box<GetRegionsResponse>),         // 220
    WebRpc(Box<EmptyResponse>),                  // 219
    GetGameList(Box<GetGameListResponse>),       // 217
}

impl PunOperationResponse {
    pub fn unparse(self, return_code: i16, debug_message: Option<String>) -> OperationResponse {
        let (operation_code, parameters) = match self {
            PunOperationResponse::Leave(p) => (operation_code::LEAVE, (*p).into()),
            PunOperationResponse::RaiseEvent(p) => (operation_code::RAISE_EVENT, (*p).into()),
            PunOperationResponse::SetProperties(p) => (operation_code::SET_PROPERTIES, (*p).into()),
            PunOperationResponse::AuthenticateOnce(p) => {
                (operation_code::AUTHENTICATE_ONCE, (*p).into())
            }
            PunOperationResponse::Authenticate(p) => (operation_code::AUTHENTICATE, (*p).into()),
            PunOperationResponse::JoinLobby(p) => (operation_code::JOIN_LOBBY, (*p).into()),
            PunOperationResponse::LeaveLobby(p) => (operation_code::LEAVE_LOBBY, (*p).into()),
            PunOperationResponse::CreateGame(p) => (operation_code::CREATE_GAME, (*p).into()),
            PunOperationResponse::JoinGame(p) => (operation_code::JOIN_GAME, (*p).into()),
            PunOperationResponse::JoinRandomGame(p) => {
                (operation_code::JOIN_RANDOM_GAME, (*p).into())
            }
            PunOperationResponse::FindFriends(p) => (operation_code::FIND_FRIENDS, (*p).into()),
            PunOperationResponse::GetRegions(p) => (operation_code::GET_REGIONS, (*p).into()),
            PunOperationResponse::WebRpc(p) => (operation_code::WEB_RPC, (*p).into()),
            PunOperationResponse::GetGameList(p) => (operation_code::GET_GAME_LIST, (*p).into()),
        };

        OperationResponse {
            operation_code,
            return_code,
            debug_message,
            parameters,
        }
    }
}

impl_u8_map_conversion! {
    AuthenticateResponse {
        [parameter_code::REPORT_QOS => PhotonObject::Boolean]
        report_qos: bool,

        [parameter_code::USER_ID => PhotonObject::String]
        user_id: String,

        // TODO: macro limitation
        // [parameter_code::ENCRYPTION_DATA => PhotonObject::Dictionary]
        // encryption_data: ((u8, u8), PhotonDictionary),

        /// The cluster when connecting to the nameserver. Not used for on-prem.
        [parameter_code::CLUSTER => PhotonObject::String]
        cluster: String,

        /// The address when connecting to the nameserver. Not used for on-prem.
        [parameter_code::ADDRESS => PhotonObject::String]
        address: String,

        // TODO: macro limitation
        // [parameter_code::ENCRYPTION_DATA => PhotonObject::Dictionary]
        // encryption_data: ((u8, u8), PhotonDictionary),

        [parameter_code::TOKEN]
        token: PhotonObject,
    }

    JoinGameResponse {
        [parameter_code::ACTOR_NR => PhotonObject::Integer]
        actor_nr: i32, // for GameServer

        [parameter_code::ACTOR_LIST => PhotonObject::Array]
        actor_list: PhotonArray, // for GameServer

        [parameter_code::PLAYER_PROPERTIES => PhotonObject::Hashtable]
        player_properties: PhotonHashmap, // for GameServer, TODO: parse (ReadoutProperties)

        [parameter_code::GAME_PROPERTIES => PhotonObject::Hashtable]
        game_properties: PhotonHashmap, // for GameServer, TODO: parse (ReadoutProperties)

        [parameter_code::ROOM_OPTION_FLAGS => PhotonObject::Integer]
        flags: i32, // for GameServer

        [parameter_code::ADDRESS => PhotonObject::String]
        address: String, // for non-GameServer

        [parameter_code::ROOM_NAME => PhotonObject::String]
        room_name: String, // for non-GameServer
    }

    FindFriendsResponse {
        @required
        [parameter_code::FIND_FRIENDS_RESPONSE_ONLINE_LIST => PhotonObject::Array]
        online_list: PhotonArray,

        @required
        [parameter_code::FIND_FRIENDS_RESPONSE_ROOM_ID_LIST => PhotonObject::Array]
        room_list: PhotonArray,
    }

    GetRegionsResponse {
        @required
        [parameter_code::REGION => PhotonObject::StringArray]
        regions: Vec<String>,

        @required
        [parameter_code::ADDRESS => PhotonObject::StringArray]
        servers: Vec<String>,
    }

    GetGameListResponse {
        @required
        [parameter_code::GAME_LIST => PhotonObject::Hashtable]
        game_list: PhotonHashmap, // TODO: parse
    }
}
