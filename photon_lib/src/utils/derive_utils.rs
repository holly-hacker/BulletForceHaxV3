use std::hash::{Hash, Hasher};

use crate::{ParameterMap, PhotonHashmap};

/// Function for use with the `derivative` crate
pub fn hash_photon_hashmap<H>(map: &PhotonHashmap, state: &mut H)
where
    H: Hasher,
{
    map.0.len().hash(state);
}

pub fn hash_parameter_map<H>(map: &ParameterMap, state: &mut H)
where
    H: Hasher,
{
    map.0.len().hash(state);
}
