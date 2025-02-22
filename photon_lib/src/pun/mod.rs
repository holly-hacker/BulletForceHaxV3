//! Implements high-level types from `PhotonRealtime`, `PhotonUnityNetworking` (PUN) and `PhotonChat`, which are the
//! default implementation of the PUN protocol.

#[macro_use]
mod macro_impl;

pub mod constants;
pub mod lifting;
mod newtypes;

pub use newtypes::*;

use std::convert::Infallible;

use thiserror::Error;

use crate::{
    ParameterMap,
    photon::{message::PhotonMessageType, object::PhotonObject},
};

#[derive(Error, Debug)]
pub enum LiftingError {
    #[error(
        "while lifting {struct_name}::{field_name} expected object type {expected_type} but got value {actual_value:?}"
    )]
    UnexpectedObjectTypeInStruct {
        /// The name of the struct that was being created
        struct_name: &'static str,
        /// The name of the field where the error occured
        field_name: &'static str,
        /// The expected PhotonObject variant
        expected_type: &'static str,
        /// The value of the encountered type
        actual_value: Box<PhotonObject>,
    },
    #[error("{0}")]
    UnexpectedObjectType(#[from] WrongPhotonObjectError),
    #[error("missing required field {struct_name}::{field_name}")]
    MissingRequiredField {
        /// The name of the struct that was being created
        struct_name: &'static str,
        /// The name of the field that was missing
        field_name: &'static str,
    },
    #[error("Unknown message code {message_code} for a {message_type:?}")]
    UnknownMessageCode {
        message_type: PhotonMessageType,
        message_code: u8,
        parameters: ParameterMap,
        operation_response_data: Option<(i16, Option<String>)>,
    },
}

impl From<Infallible> for LiftingError {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Error, Debug)]
#[error("expected object type {expected_type} but got value {actual_value:?}")]
pub struct WrongPhotonObjectError {
    /// The expected PhotonObject variant
    pub expected_type: &'static str,
    /// The value of the encountered type
    pub actual_value: Box<PhotonObject>,
}
