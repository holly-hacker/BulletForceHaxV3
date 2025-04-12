use std::borrow::Cow;

use photon_lib::{ReadError, WriteError, pun::LiftingError};

#[derive(thiserror::Error, Debug)]
pub enum LobbyError {
    #[error("failed to parse message: {0}")]
    PhotonReadError(#[from] ReadError),
    #[error("failed to interpreting message: {0}")]
    PhotonLiftingError(#[from] LiftingError),
    #[error("failed to write message: {0}")]
    PhotoWriteError(#[from] WriteError),
    #[error("{0}")]
    Other(Cow<'static, str>),
}
