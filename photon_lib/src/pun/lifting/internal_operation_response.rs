use crate::{
    photon::{
        message::{OperationResponse, PhotonMessageType},
        object::PhotonObject,
    },
    pun::{LiftingError, constants::*},
};

pub trait ParseInternalOperationResponseExt {
    fn parse_internal(
        self,
    ) -> Result<(PunInternalOperationResponse, i16, Option<String>), LiftingError>;
}

impl ParseInternalOperationResponseExt for OperationResponse {
    fn parse_internal(
        self,
    ) -> Result<(PunInternalOperationResponse, i16, Option<String>), LiftingError> {
        Ok((
            match self.operation_code {
                internal_operation_code::INIT_ENCRYPTION => {
                    PunInternalOperationResponse::InitEncryption(Box::new(
                        InitEncryptionResponse::try_from(self.parameters)?,
                    ))
                }
                internal_operation_code::PING => PunInternalOperationResponse::Ping(Box::new(
                    PingResponse::try_from(self.parameters)?,
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
pub enum PunInternalOperationResponse {
    InitEncryption(Box<InitEncryptionResponse>), // 0
    Ping(Box<PingResponse>),                     // 1
}

impl PunInternalOperationResponse {
    pub fn unparse(self, return_code: i16, debug_message: Option<String>) -> OperationResponse {
        let (operation_code, parameters) = match self {
            PunInternalOperationResponse::InitEncryption(p) => {
                (internal_operation_code::INIT_ENCRYPTION, (*p).into())
            }
            PunInternalOperationResponse::Ping(p) => (internal_operation_code::PING, (*p).into()),
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
    PingResponse {
        @required
        [parameter_code::PONG_CLIENT_TIME => PhotonObject::Integer]
        client_time: i32,

        @required
        [parameter_code::PONG_SERVER_TIME => PhotonObject::Integer]
        server_time: i32,
    }

    InitEncryptionResponse {
        /// The server's public key
        @required
        [parameter_code::INIT_ENCRYPTION_SERVER_KEY => PhotonObject::ByteArray]
        public_key: Vec<u8>,
    }
}
