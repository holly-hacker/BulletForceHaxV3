//! Implements high-level types from PhotonRealtime, PhotonUnityNetworking and PhotonChat

#[macro_use]
mod macro_impl;

pub mod constants;
pub mod structs;
mod structs_impl;

use std::convert::Infallible;

use thiserror::Error;

// TODO: proper error type
#[derive(Error, Debug)]
#[error("{0}")]
pub struct FromMapError(pub String);

impl From<Infallible> for FromMapError {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}
