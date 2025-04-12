use crate::{
    photon::{
        message::{OperationRequest, PhotonMessageType},
        object::PhotonObject,
    },
    pun::{LiftingError, constants::*},
};

pub trait ParseInternalOperationRequestExt {
    fn parse_internal(self) -> Result<PunInternalOperationRequest, LiftingError>;
}

impl ParseInternalOperationRequestExt for OperationRequest {
    fn parse_internal(self) -> Result<PunInternalOperationRequest, LiftingError> {
        Ok(match self.operation_code {
            internal_operation_code::INIT_ENCRYPTION => {
                PunInternalOperationRequest::InitEncryption(Box::new(
                    InitEncryptionRequest::try_from(self.parameters)?,
                ))
            }
            internal_operation_code::PING => {
                PunInternalOperationRequest::Ping(Box::new(PingRequest::try_from(self.parameters)?))
            }

            _ => {
                return Err(LiftingError::UnknownMessageCode {
                    message_type: PhotonMessageType::InternalOperationRequest,
                    message_code: self.operation_code,
                    parameters: self.parameters,
                    operation_response_data: None,
                });
            }
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum PunInternalOperationRequest {
    InitEncryption(Box<InitEncryptionRequest>), // 0
    Ping(Box<PingRequest>),                     // 1
}

impl PunInternalOperationRequest {
    pub fn unparse(self) -> OperationRequest {
        let (operation_code, parameters) = match self {
            PunInternalOperationRequest::InitEncryption(p) => {
                (internal_operation_code::INIT_ENCRYPTION, (*p).into())
            }
            PunInternalOperationRequest::Ping(p) => (internal_operation_code::PING, (*p).into()),
        };

        OperationRequest {
            operation_code,
            parameters,
        }
    }
}

impl_u8_map_conversion! {
    /// A client-side ping request. Only used on platforms that do their own framing (ie. WebSocket).
    PingRequest {
        /// Client-side tick count. Typically time since system start (`Environment.TickCount`).
        @required
        [parameter_code::PING_CLIENT_TIME => PhotonObject::Integer]
        client_time: i32,
    }

    InitEncryptionRequest {
        /// The client's public key
        @required
        [parameter_code::INIT_ENCRYPTION_CLIENT_KEY => PhotonObject::ByteArray]
        client_key: Vec<u8>,
    }
}
