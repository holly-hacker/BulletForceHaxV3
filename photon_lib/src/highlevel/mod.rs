//! Implements high-level types from PhotonRealtime, PhotonUnityNetworking and PhotonChat

#[macro_use]
mod macro_impl;

pub mod constants;
pub mod structs;
mod structs_impl;

use std::convert::Infallible;

use thiserror::Error;

use crate::photon_data_type::PhotonDataType;

#[derive(Error, Debug)]
pub enum LiftingError {
    #[error("while lifting {struct_name}::{field_name} expected data type {expected_type} but got value {actual_value:?}")]
    UnexpectedDataTypeInStruct {
        /// The name of the struct that was being created
        struct_name: &'static str,
        /// The name of the field where the error occured
        field_name: &'static str,
        /// The expected PhotonDataType variant
        expected_type: &'static str,
        /// The value of the encountered type
        actual_value: PhotonDataType,
    },
    #[error("{0}")]
    UnexpectedDataType(#[from] WrongPhotonDataTypeError),
    #[error("missing required field {struct_name}::{field_name}")]
    MissingRequiredField {
        /// The name of the struct that was being created
        struct_name: &'static str,
        /// The name of the field that was missing
        field_name: &'static str,
    },
}

impl From<Infallible> for LiftingError {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Error, Debug)]
#[error("expected data type {expected_type} but got value {actual_value:?}")]
pub struct WrongPhotonDataTypeError {
    /// The expected PhotonDataType variant
    pub expected_type: &'static str,
    /// The value of the encountered type
    pub actual_value: PhotonDataType,
}
