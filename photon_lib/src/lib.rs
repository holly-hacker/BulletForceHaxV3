//! This library aims to help with parsing Photon Unity Networking network packets.

#[macro_use]
pub mod pun;
pub mod photon;
pub mod primitives;

use std::hash::Hash;

pub use indexmap;
use indexmap::IndexMap;
pub use ordered_float;
use photon::object::PhotonObject;
use pun::LiftingError;
use thiserror::Error;

// TODO: perhaps add info on where the error occured?
macro_rules! check_remaining {
    ($bytes:ident, $min_bytes:expr) => {
        if $bytes.remaining() < $min_bytes {
            return Err(ReadError::NotEnoughBytesLeft);
        }
    };
}

pub(crate) use check_remaining;

/// A newtype for a hashmap containing photon-serialized objects
#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhotonHashmap(IndexMap<PhotonObject, PhotonObject>);

impl std::hash::Hash for PhotonHashmap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|k| k.hash(state));
    }
}

impl<
    K: TryFrom<PhotonObject, Error = impl Into<LiftingError>> + Hash + Eq,
    V: TryFrom<PhotonObject, Error = impl Into<LiftingError>>,
> TryFrom<PhotonHashmap> for IndexMap<K, V>
{
    type Error = LiftingError;

    fn try_from(value: PhotonHashmap) -> Result<Self, Self::Error> {
        let mut ret = Self::new();

        for (k, v) in value.0.into_iter() {
            let k = K::try_from(k).map_err(|e| e.into())?;
            let v = V::try_from(v).map_err(|e| e.into())?;
            ret.insert(k, v);
        }

        Ok(ret)
    }
}

impl<K: Into<PhotonObject>, V: Into<PhotonObject>> From<IndexMap<K, V>> for PhotonHashmap {
    fn from(value: IndexMap<K, V>) -> Self {
        PhotonHashmap(
            value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

/// A newtype for a hashmap containing photon-serialized objects
#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhotonDictionary(IndexMap<PhotonObject, PhotonObject>);

impl std::hash::Hash for PhotonDictionary {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|k| k.hash(state));
    }
}

/// A newtype for the parameter hashmap used in photon messages.
#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterMap(IndexMap<u8, PhotonObject>);

impl std::hash::Hash for ParameterMap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|k| k.hash(state));
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PhotonArray(Vec<PhotonObject>);

impl From<Vec<PhotonObject>> for PhotonArray {
    fn from(value: Vec<PhotonObject>) -> Self {
        Self(value)
    }
}

impl From<PhotonArray> for Vec<PhotonObject> {
    fn from(value: PhotonArray) -> Self {
        value.0
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PhotonObjectArray(Vec<PhotonObject>);

impl From<Vec<PhotonObject>> for PhotonObjectArray {
    fn from(value: Vec<PhotonObject>) -> Self {
        Self(value)
    }
}

impl From<PhotonObjectArray> for Vec<PhotonObject> {
    fn from(value: PhotonObjectArray) -> Self {
        value.0
    }
}

/// An error that can occur when reading a message
#[derive(Debug, Error)]
pub enum ReadError {
    #[error("not enough bytes left in the buffer")]
    NotEnoughBytesLeft,
    #[error("unexpected data was found: {0}")]
    UnexpectedData(&'static str),
    #[error("packet contained invalid magic number: {0:#02X}")]
    InvalidMagicNumber(u8),
    #[error("message type is unknown: {0:#02X}")]
    UnknownMessageType(u8),
    #[error("object type is unknown: {0:#02X}")]
    UnknownObjectType(u8),
    #[error("the following functionality is not yet implemented: {0}")]
    Unimplemented(&'static str),
    #[error("invalid length for custom data {0}, expected {1} but found {2}")]
    CustomDataInvalidLength(&'static str, usize, usize),
}

/// An error that can occur when writing a message
#[derive(Debug, Error)]
pub enum WriteError {
    // TODO: NotEnoughBytesLeft error, we currently panic if we write to a fixed-size buffer
    #[error("Items in array were not all of the same type")]
    UnhomogeneousArray,
    #[error("Key or value in typed dictionary did not match")]
    TypeMismatchInTypedDictionary,
    #[error("Value was too large: {0}")]
    ValueTooLarge(&'static str),
    #[error("the following functionality is not yet implemented: {0}")]
    Unimplemented(&'static str),
}
