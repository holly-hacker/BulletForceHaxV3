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
