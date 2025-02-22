use serde::{Deserialize, Serialize};

const PHOTON_NETWORK_MAX_VIEW_IDS: i32 = 1000;

/// Represents a Photon View ID
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewId(pub i32);

impl ViewId {
    pub fn get_owner_id(&self) -> i32 {
        self.0 / PHOTON_NETWORK_MAX_VIEW_IDS
    }
}

impl From<i32> for ViewId {
    fn from(value: i32) -> Self {
        ViewId(value)
    }
}

impl From<ViewId> for i32 {
    fn from(value: ViewId) -> Self {
        value.0
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActorNr(pub i32);
