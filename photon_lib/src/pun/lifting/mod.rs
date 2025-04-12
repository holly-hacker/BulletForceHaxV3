//! Code and types for lifting Photon messages to PUN types

mod events;
mod internal_operation_request;
mod internal_operation_response;
mod operation_request;
mod operation_response;

pub use events::*;
pub use internal_operation_request::*;
pub use internal_operation_response::*;
pub use operation_request::*;
pub use operation_response::*;

impl_u8_map_conversion! {
    EmptyResponse { }
}
