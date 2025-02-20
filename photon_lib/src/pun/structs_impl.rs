use super::{LiftingError, structs::*};
use crate::{PhotonHashmap, photon::object::PhotonObject};

const PHOTON_NETWORK_MAX_VIEW_IDS: i32 = 1000;

impl ViewId {
    pub fn get_owner_id(&self) -> i32 {
        self.0 / PHOTON_NETWORK_MAX_VIEW_IDS
    }
}

impl RpcEvent {
    /// Drains the [Self::data] field
    pub fn into_rpc_call(self) -> Result<RpcCall, LiftingError> {
        RpcCall::try_from(self.data)
    }
}

impl SendSerializeEvent {
    // TODO: proper error type. probably want something generic like InvalidDataError
    /// Gets a copy of the serialized data in this event
    pub fn get_serialized_data(&self) -> Option<Vec<SerializedData>> {
        Self::parse_serialized_data(&self.data)
    }

    pub fn parse_serialized_data(data: &PhotonHashmap) -> Option<Vec<SerializedData>> {
        _ = data.0.get(&PhotonObject::Byte(1))?;

        let header_len = match data.0.contains_key(&PhotonObject::Byte(1)) {
            true => 2,
            false => 1,
        };

        const DATA_INITIAL_INDEX: usize = 10;
        let data_len = data.0.len() - header_len;
        let mut ret = vec![];
        for i in 0..data_len {
            // items start at key 10 and count up
            // the official implementation has various opportunities to crash here, but we should be safe
            let index = i + DATA_INITIAL_INDEX;
            let index = (index & 0xFF) as u8; // NOTE: official implementation does wrap here

            let found = data.0.get(&PhotonObject::Byte(index));
            let found = match found {
                Some(PhotonObject::ObjectArray(x)) => x,
                _ => return None,
            };
            let data = SerializedData::from_object_array(found.clone())?;
            ret.push(data);
        }

        Some(ret)
    }
}

impl SerializedData {
    // TODO: views can have specific synchronisation. User should pass that to this method
    // TODO: better errors
    pub fn from_object_array(mut data: Vec<PhotonObject>) -> Option<Self> {
        if data.len() < 3 {
            return None;
        }

        let view_id = match data[0] {
            PhotonObject::Integer(i) => i,
            _ => return None,
        };
        // index 1 and 2 are related to compression, which is not implemented yet
        let data_stream = data.drain(3..).collect();

        Some(Self {
            view_id,
            data_stream,
        })
    }

    pub fn get_view_id(&self) -> ViewId {
        ViewId(self.view_id)
    }

    // NOTE: could add an implementation to parse this component as a Transform, Rigidbody or Rigidbody2D, as they have
    // pre-defined data streams. The user would have to pass their `onSerializeTransformOption` or
    // `onSerializeRigidBodyOption` value, though.
}

impl DestroyEventData {
    pub fn get_view_id(&self) -> ViewId {
        ViewId(self.view_id)
    }
}

impl InstantiationEventData {
    pub fn get_view_id(&self) -> ViewId {
        ViewId(self.instantiation_id)
    }
}

impl RpcCall {
    pub fn get_view_id(&self) -> ViewId {
        ViewId(self.net_view_id)
    }
}
