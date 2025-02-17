//! Implements high-level types from PhotonRealtime, PhotonUnityNetworking and PhotonChat

#[macro_use]
mod macro_impl;

pub mod constants;
pub mod structs;
mod structs_impl;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct FromMapError(pub String);
