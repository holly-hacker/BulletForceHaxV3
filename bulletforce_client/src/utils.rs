use photon_lib::{
    ParameterMap,
    photon::message::{OperationRequest, PhotonMessage},
};

pub fn to_operation_request(
    operation_code: u8,
    request_object: impl Into<ParameterMap>,
) -> PhotonMessage {
    PhotonMessage::OperationRequest(OperationRequest {
        operation_code,
        parameters: request_object.into(),
    })
}

pub fn to_internal_operation_request(
    internal_operation_code: u8,
    request_object: impl Into<ParameterMap>,
) -> PhotonMessage {
    PhotonMessage::InternalOperationRequest(OperationRequest {
        operation_code: internal_operation_code,
        parameters: request_object.into(),
    })
}
