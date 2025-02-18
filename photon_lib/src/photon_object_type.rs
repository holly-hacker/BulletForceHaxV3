//! Implements the low-level object types and their (de)serialization

use std::cmp::Ordering;

use bytes::{Buf, BufMut};
use ordered_float::OrderedFloat;

use crate::{
    check_remaining,
    highlevel::WrongPhotonObjectError,
    photon_message::{EventData, OperationRequest, OperationResponse},
    primitives::*,
    PhotonDictionary, PhotonHashmap, ReadError, WriteError,
};

/// A serialized .NET object
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PhotonObject {
    #[default]
    /// Object type 0x2A, represents .NET's `null`
    Null,
    /// Object type 0x44, holds a `Dictionary<TKey, TValue>`. Because this dictionary is generic, we need to store the key and value kind as well.
    Dictionary((u8, u8), PhotonDictionary),
    /// Object type 0x61, holds a `string[]`.
    StringArray(Vec<String>),
    /// Object type 0x62, holds a `byte`
    Byte(u8),
    /// Object type 0x63, holds an `object`. This uses a deserialization function that is provided by the game.
    Custom(CustomData),
    /// Object type 0x64, holds a `double`
    Double(OrderedFloat<f64>),
    /// Object type 0x65, holds [EventData]
    EventData(EventData),
    /// Object type 0x66, holds a `float`
    Float(OrderedFloat<f32>),
    /// Object type 0x68, holds a photon Hashtable. This hashtable aims to mimic `System.Collections.Hashtable`.
    Hashtable(PhotonHashmap),
    /// Object type 0x69, holds an `int`
    Integer(i32),
    /// Object type 0x6B, holds a `short`
    Short(i16),
    /// Object type 0x6C, holds a `long`
    Long(i64),
    /// Object type 0x6E, holds an `int[]`
    IntArray(Vec<i32>),
    /// Object type 0x6F, holds a `bool`
    Boolean(bool),
    /// Object type 0x70, holds an [OperationResponse]
    OperationResponse(OperationResponse),
    /// Object type 0x71, holds an [OperationRequest]
    OperationRequest(OperationRequest),
    /// Object type 0x73, holds a `string`
    String(String),
    /// Object type 0x78, holds a `byte[]`
    ByteArray(Vec<u8>),
    /// Object type 0x79, holds an `Array`. Elements must be of the same type.
    Array(Vec<PhotonObject>),
    /// Object type 0x7A, holds an `object[]`
    ObjectArray(Vec<PhotonObject>),
}

macro_rules! impl_from {
    ($($variant:ident => $type:ty,)*) => {
        $(
            impl From<$type> for PhotonObject {
                fn from(value: $type) -> Self {
                    Self::$variant(value)
                }
            }

            impl TryFrom<PhotonObject> for $type {
                type Error = WrongPhotonObjectError;

                fn try_from(value: PhotonObject) -> Result<Self, Self::Error> {
                    if let PhotonObject::$variant(str) = value {
                        Ok(str)
                    } else {
                        Err(WrongPhotonObjectError {
                            expected_type: stringify!($variant),
                            actual_value: Box::new(value),
                        })
                    }
                }
            }
        )*
    };
}

// TODO: Null, Dictionary, Array, ObjectArray
// for null, not sure what makes sense. Maybe `()`?
// for dictionary, maybe try to get rid of the `(u8, u8)`
// for array/objectarray, probably want more newtypes
impl_from! {
    StringArray => Vec<String>,
    Byte => u8,
    Custom => CustomData,
    Double => OrderedFloat<f64>,
    EventData => EventData,
    Float => OrderedFloat<f32>,
    Hashtable => PhotonHashmap,
    Integer => i32,
    Short => i16,
    Long => i64,
    IntArray => Vec<i32>,
    Boolean => bool,
    OperationResponse => OperationResponse,
    OperationRequest => OperationRequest,
    String => String,
    ByteArray => Vec<u8>,
}

impl PhotonObject {
    pub fn from_bytes(bytes: &mut impl Buf) -> Result<PhotonObject, ReadError> {
        check_remaining!(bytes, 1);

        let object_type = bytes.get_u8();
        Self::from_bytes_with_type(bytes, object_type)
    }

    pub fn from_bytes_with_type(
        bytes: &mut impl Buf,
        object_type: u8,
    ) -> Result<PhotonObject, ReadError> {
        match object_type {
            0 | 0x2A => Ok(PhotonObject::Null), // NOTE: 0 = unknown
            0x44 => {
                check_remaining!(bytes, 4);
                // NOTE: implementation does not allow 0x44 or 0x69 as key or value
                let key_type = bytes.get_u8();
                let val_type = bytes.get_u8();
                let len = bytes.get_i16();

                let read_key = key_type == 0 || key_type == 0x2A;
                let read_val = val_type == 0 || val_type == 0x2A;

                let mut map = PhotonDictionary::default();
                for _ in 0..len {
                    let key = match read_key {
                        true => Self::from_bytes(bytes)?,
                        false => Self::from_bytes_with_type(bytes, key_type)?,
                    };
                    let val = match read_val {
                        true => Self::from_bytes(bytes)?,
                        false => Self::from_bytes_with_type(bytes, val_type)?,
                    };

                    if key != PhotonObject::Null {
                        map.0.insert(key, val);
                    }
                }

                Ok(PhotonObject::Dictionary((key_type, val_type), map))
            }
            0x61 => {
                check_remaining!(bytes, 2);
                let len = bytes.get_i16();
                let v = if len > 0 {
                    let mut v = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        match Self::from_bytes_with_type(bytes, 0x73)? {
                            PhotonObject::String(s) => v.push(s),
                            _ => unreachable!(),
                        }
                    }
                    v
                } else {
                    vec![]
                };
                Ok(PhotonObject::StringArray(v))
            }
            0x62 => {
                check_remaining!(bytes, 1);
                Ok(PhotonObject::Byte(bytes.get_u8()))
            }
            0x63 => Ok(PhotonObject::Custom(CustomData::from_bytes(bytes)?)),
            0x64 => {
                check_remaining!(bytes, 8);
                Ok(PhotonObject::Double(bytes.get_f64().into()))
            }
            0x65 => Ok(PhotonObject::EventData(EventData::from_bytes(bytes)?)),
            0x66 => {
                check_remaining!(bytes, 4);
                Ok(PhotonObject::Float(bytes.get_f32().into()))
            }
            0x68 => {
                check_remaining!(bytes, 2);
                // NOTE: implementation does not allow 0x44 or 0x69 as key or value
                let len = bytes.get_i16();

                let mut map = PhotonHashmap::default();
                for _ in 0..len {
                    let key = Self::from_bytes(bytes)?;
                    let val = Self::from_bytes(bytes)?;

                    if key != PhotonObject::Null {
                        map.0.insert(key, val);
                    }
                }

                Ok(PhotonObject::Hashtable(map))
            }
            0x69 => {
                check_remaining!(bytes, 4);
                Ok(PhotonObject::Integer(bytes.get_i32()))
            }
            0x6B => {
                check_remaining!(bytes, 2);
                Ok(PhotonObject::Short(bytes.get_i16()))
            }
            0x6C => {
                check_remaining!(bytes, 8);
                Ok(PhotonObject::Long(bytes.get_i64()))
            }
            0x6E => {
                check_remaining!(bytes, 4);
                let len = bytes.get_i32();
                let v = if len > 0 {
                    let mut v = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        check_remaining!(bytes, 4);
                        v.push(bytes.get_i32());
                    }
                    v
                } else {
                    vec![]
                };
                Ok(PhotonObject::IntArray(v))
            }
            0x6F => {
                check_remaining!(bytes, 1);
                Ok(PhotonObject::Boolean(bytes.get_u8() != 0))
            }
            0x70 => Ok(PhotonObject::OperationResponse(
                OperationResponse::from_bytes(bytes)?,
            )),
            0x71 => Ok(PhotonObject::OperationRequest(
                OperationRequest::from_bytes(bytes)?,
            )),
            0x73 => {
                check_remaining!(bytes, 2);
                let len = bytes.get_i16();
                let str = match len.cmp(&0) {
                    Ordering::Greater => {
                        check_remaining!(bytes, len as usize);
                        let mut buffer = vec![0u8; len as usize];
                        bytes.copy_to_slice(&mut buffer);

                        // NOTE: System.Text.Encoding.UTF8.GetString will replace invalid unicode with �, so we imitate
                        // that behavior.
                        let str = String::from_utf8_lossy(&buffer);
                        str.to_string()
                    }
                    Ordering::Equal => String::new(),
                    // this seems inconsistent with other branches but this is what the original code would do
                    Ordering::Less => {
                        return Err(ReadError::UnexpectedData("string length less than 0"));
                    }
                };

                Ok(PhotonObject::String(str))
            }
            0x78 => {
                check_remaining!(bytes, 4);
                let len = bytes.get_i32();
                if len < 0 {
                    return Err(ReadError::UnexpectedData("byte[] length less than 0"));
                }

                check_remaining!(bytes, len as usize);
                let mut v = vec![0u8; len as usize];
                bytes.copy_to_slice(&mut v);

                Ok(PhotonObject::ByteArray(v))
            }
            0x79 => {
                check_remaining!(bytes, 3);
                let len = bytes.get_i16();
                let object_type = bytes.get_u8();

                let v = if len > 0 {
                    let mut vec = Vec::with_capacity(len as usize);

                    for _ in 0..len {
                        vec.push(Self::from_bytes_with_type(bytes, object_type)?);
                    }

                    vec
                } else {
                    vec![]
                };

                Ok(PhotonObject::Array(v))
            }
            0x7A => {
                check_remaining!(bytes, 2);
                let len = bytes.get_i16();

                if len < 0 {
                    return Err(ReadError::UnexpectedData("object[] length less than 0"));
                }

                let mut v = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    v.push(Self::from_bytes(bytes)?);
                }

                Ok(PhotonObject::ObjectArray(v))
            }
            _ => Err(ReadError::UnknownObjectType(object_type)),
        }
    }

    pub fn to_bytes(&self, buf: &mut impl BufMut) -> Result<(), WriteError> {
        buf.put_u8(self.get_type_byte());

        self.to_bytes_without_type_byte(buf)
    }

    pub fn to_bytes_without_type_byte(&self, buf: &mut impl BufMut) -> Result<(), WriteError> {
        match self {
            PhotonObject::Null => (),
            PhotonObject::Dictionary((key_type, val_type), d) => {
                buf.put_u8(*key_type);
                buf.put_u8(*val_type);

                if d.0.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("Custom Data"));
                }

                buf.put_i16(d.0.len() as i16);

                let write_key = *key_type == 0 || *key_type == 0x2A;
                let write_val = *val_type == 0 || *val_type == 0x2A;

                for (k, v) in &d.0 {
                    match write_key {
                        true => k.to_bytes(buf)?,
                        false if k.get_type_byte() != *key_type => {
                            return Err(WriteError::TypeMismatchInTypedDictionary)
                        }
                        false => k.to_bytes_without_type_byte(buf)?,
                    };

                    match write_val {
                        true => v.to_bytes(buf)?,
                        false if v.get_type_byte() != *val_type => {
                            return Err(WriteError::TypeMismatchInTypedDictionary)
                        }
                        false => v.to_bytes_without_type_byte(buf)?,
                    };
                }
            }
            PhotonObject::StringArray(a) => {
                if a.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("Custom Data"));
                }
                buf.put_i16(a.len() as i16);

                for s in a {
                    let len = s.len();
                    if len > i16::MAX as usize {
                        return Err(WriteError::ValueTooLarge("String"));
                    }
                    buf.put_i16(len as i16);
                    buf.put_slice(s.as_bytes());
                }
            }
            PhotonObject::Byte(b) => buf.put_u8(*b),
            PhotonObject::Custom(data) => data.to_bytes(buf)?,
            PhotonObject::Double(d) => buf.put_f64(d.0),
            PhotonObject::EventData(d) => d.to_bytes(buf)?,
            PhotonObject::Float(f) => buf.put_f32(f.0),
            PhotonObject::Hashtable(PhotonHashmap(t)) => {
                if t.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("String"));
                }

                buf.put_i16(t.len() as i16);

                for (k, v) in t {
                    k.to_bytes(buf)?;
                    v.to_bytes(buf)?;
                }
            }
            PhotonObject::Integer(i) => buf.put_i32(*i),
            PhotonObject::Short(s) => buf.put_i16(*s),
            PhotonObject::Long(l) => buf.put_i64(*l),
            PhotonObject::IntArray(v) => {
                if v.len() > i32::MAX as usize {
                    return Err(WriteError::ValueTooLarge("String"));
                }
                buf.put_i32(v.len() as i32);

                for &i in v {
                    buf.put_i32(i);
                }
            }
            PhotonObject::Boolean(b) => buf.put_u8(if *b { 1 } else { 0 }),
            PhotonObject::OperationResponse(r) => r.to_bytes(buf)?,
            PhotonObject::OperationRequest(r) => r.to_bytes(buf)?,
            PhotonObject::String(s) => {
                let len = s.len();
                if len > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("String"));
                }
                buf.put_i16(len as i16);
                buf.put_slice(s.as_bytes());
            }
            PhotonObject::ByteArray(v) => {
                if v.len() > i32::MAX as usize {
                    return Err(WriteError::ValueTooLarge("ByteArray"));
                }
                buf.put_i32(v.len() as i32);

                buf.put_slice(v);
            }
            PhotonObject::Array(v) => {
                if v.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("Array"));
                }
                buf.put_i16(v.len() as i16);

                let type_byte = match v.first() {
                    Some(i) => i.get_type_byte(),
                    None => PhotonObject::Null.get_type_byte(),
                };
                buf.put_u8(type_byte);

                for item in v {
                    if item.get_type_byte() != type_byte {
                        return Err(WriteError::UnhomogeneousArray);
                    }
                    item.to_bytes_without_type_byte(buf)?;
                }
            }
            PhotonObject::ObjectArray(v) => {
                if v.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("ObjectArray"));
                }
                buf.put_i16(v.len() as i16);

                for item in v {
                    item.to_bytes(buf)?;
                }
            }
        }

        Ok(())
    }

    pub fn get_type_byte(&self) -> u8 {
        match self {
            PhotonObject::Null => 0x2A,
            PhotonObject::Dictionary(_, _) => 0x44,
            PhotonObject::StringArray(_) => 0x61,
            PhotonObject::Byte(_) => 0x62,
            PhotonObject::Custom(_) => 0x63,
            PhotonObject::Double(_) => 0x64,
            PhotonObject::EventData(_) => 0x65,
            PhotonObject::Float(_) => 0x66,
            PhotonObject::Hashtable(_) => 0x68,
            PhotonObject::Integer(_) => 0x69,
            PhotonObject::Short(_) => 0x6B,
            PhotonObject::Long(_) => 0x6C,
            PhotonObject::IntArray(_) => 0x6E,
            PhotonObject::Boolean(_) => 0x6F,
            PhotonObject::OperationResponse(_) => 0x70,
            PhotonObject::OperationRequest(_) => 0x71,
            PhotonObject::String(_) => 0x73,
            PhotonObject::ByteArray(_) => 0x78,
            PhotonObject::Array(_) => 0x79,
            PhotonObject::ObjectArray(_) => 0x7A,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomData {
    Vector2(Vector2),
    Vector3(Vector3),
    Quaternion(Quaternion),
    PhotonPlayer(i32),
    Unrecognized(u8, Vec<u8>),
}

impl CustomData {
    pub fn to_bytes(&self, buf: &mut impl BufMut) -> Result<(), WriteError> {
        match self {
            CustomData::Vector2(Vector2(OrderedFloat(x), OrderedFloat(y))) => {
                buf.put_u8(b'W');
                buf.put_i16(8);
                buf.put_f32(*x);
                buf.put_f32(*y);
            }
            CustomData::Vector3(Vector3(OrderedFloat(x), OrderedFloat(y), OrderedFloat(z))) => {
                buf.put_u8(b'V');
                buf.put_i16(12);
                buf.put_f32(*x);
                buf.put_f32(*y);
                buf.put_f32(*z);
            }
            CustomData::Quaternion(Quaternion(
                OrderedFloat(w),
                OrderedFloat(x),
                OrderedFloat(y),
                OrderedFloat(z),
            )) => {
                buf.put_u8(b'Q');
                buf.put_i16(16);
                buf.put_f32(*w);
                buf.put_f32(*x);
                buf.put_f32(*y);
                buf.put_f32(*z);
            }
            CustomData::PhotonPlayer(i) => {
                buf.put_u8(b'P');
                buf.put_i16(4);
                buf.put_i32(*i);
            }
            CustomData::Unrecognized(type_code, v) => {
                buf.put_u8(*type_code);

                if v.len() > i16::MAX as usize {
                    return Err(WriteError::ValueTooLarge("Custom Data"));
                }

                buf.put_i16(v.len() as i16);
                buf.put_slice(v);
            }
        }
        Ok(())
    }

    pub fn from_bytes(bytes: &mut impl Buf) -> Result<Self, ReadError> {
        check_remaining!(bytes, 3);
        let type_code = bytes.get_u8();

        let len = bytes.get_i16();
        if len < 0 {
            return Err(ReadError::UnexpectedData("negative length for custom data"));
        }
        let len = len as usize;

        check_remaining!(bytes, len);

        match type_code {
            b'W' => {
                if len != 8 {
                    return Err(ReadError::CustomDataInvalidLength("Vector2", 8, len));
                }
                Ok(CustomData::Vector2(Vector2(
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                )))
            }
            b'V' => {
                if len != 12 {
                    return Err(ReadError::CustomDataInvalidLength("Vector3", 12, len));
                }
                Ok(CustomData::Vector3(Vector3(
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                )))
            }
            b'Q' => {
                if len != 16 {
                    return Err(ReadError::CustomDataInvalidLength("Quaternion", 16, len));
                }
                Ok(CustomData::Quaternion(Quaternion(
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                    OrderedFloat(bytes.get_f32()),
                )))
            }
            b'P' => {
                if len != 4 {
                    return Err(ReadError::CustomDataInvalidLength("PhotonPlayer", 4, len));
                }
                Ok(CustomData::PhotonPlayer(bytes.get_i32()))
            }
            _ => {
                check_remaining!(bytes, len);
                let mut v = vec![0u8; len];
                bytes.copy_to_slice(&mut v);
                Ok(CustomData::Unrecognized(type_code, v))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{photon_message::*, photon_object_type::*, ParameterMap};

    macro_rules! generate_test {
        ($name: ident, $val: expr, $hex: expr) => {
            paste::paste! {
                #[test]
                fn [<deserialize_ $name>]() {
                    let mut bytes: &[u8] = &hex::decode($hex).expect("valid hex data in test");
                    let val = $val;

                    let deserialized = super::PhotonObject::from_bytes(&mut bytes).unwrap();

                    assert_eq!(deserialized, val);
                }

                #[test]
                fn [<serialize_ $name>]() {
                    use super::PhotonObject;

                    let bytes = hex::decode($hex).expect("valid hex data in test");
                    let val = $val;

                    let mut buf = vec![];
                    val.to_bytes(&mut buf).unwrap();

                    assert_eq!(buf, bytes);
                }
            }
        };
    }

    #[test]
    fn deserialize_00_to_null() {
        assert_eq!(
            PhotonObject::from_bytes(&mut bytes::Bytes::from(&[0x00u8][..])).unwrap(),
            PhotonObject::Null
        );
    }

    generate_test!(null, PhotonObject::Null, "2a");
    generate_test!(bool_true, PhotonObject::Boolean(true), "6f01");
    generate_test!(bool_false, PhotonObject::Boolean(false), "6f00");
    generate_test!(u8, PhotonObject::Byte(0x90), "6290");
    generate_test!(s16, PhotonObject::Short(-1337), "6BFAC7");
    generate_test!(s32, PhotonObject::Integer(-559038737), "69DEADBEEF");
    generate_test!(
        s64,
        PhotonObject::Long(-3886136854700967234),
        "6cCA11AB1ECAFEBABE"
    );
    generate_test!(f32, PhotonObject::Float(42f32.into()), "6642280000");
    generate_test!(
        f64,
        PhotonObject::Double(13.37f64.into()),
        "64402abd70a3d70a3d"
    );
    generate_test!(string, PhotonObject::String("abc".into()), "730003616263");
    generate_test!(
        string_unicode,
        PhotonObject::String("abc»d".into()),
        "730006616263c2bb64"
    );
    generate_test!(
        byte_array,
        PhotonObject::ByteArray(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        "7800000004DEADBEEF"
    );
    generate_test!(
        int_array,
        PhotonObject::IntArray(vec![-559038737, -889275714]),
        "6E00000002DEADBEEFCAFEBABE"
    );
    generate_test!(
        string_array,
        PhotonObject::StringArray(vec!["abc".into(), "".into()]),
        "61000200036162630000"
    );
    generate_test!(
        array,
        PhotonObject::Array(vec![
            PhotonObject::Boolean(true),
            PhotonObject::Boolean(false),
            PhotonObject::Boolean(true)
        ]),
        "7900036F010001"
    );
    generate_test!(
        object_array,
        PhotonObject::ObjectArray(vec![
            PhotonObject::String("abc".into()),
            PhotonObject::Null,
            PhotonObject::Short(0x123)
        ]),
        "7A00037300036162632A6B0123"
    );

    // hashtable can only have 1 item because order is not deterministic
    generate_test!(
        hashtable,
        PhotonObject::Hashtable(PhotonHashmap(
            indexmap::indexmap! { PhotonObject::Byte(0xFF) => PhotonObject::Null, }
        )),
        "68000162FF2A"
    );

    generate_test!(
        dictionary_byte_string,
        PhotonObject::Dictionary(
            (0x62, 0x73),
            PhotonDictionary(indexmap::indexmap! {
                PhotonObject::Byte(0x01) => PhotonObject::String("one".into()),
                PhotonObject::Byte(0x02) => PhotonObject::String("two".into()),
            })
        ),
        "44627300020100036f6e6502000374776f"
    );

    generate_test!(
        dictionary_untyped,
        PhotonObject::Dictionary(
            (0, 0),
            PhotonDictionary(indexmap::indexmap! {
                PhotonObject::Byte(0x00) => PhotonObject::Short(0x1234),
                PhotonObject::String("a".into()) => PhotonObject::Byte(0xFF),
            })
        ),
        "440000000262006B12347300016162FF"
    );

    generate_test!(
        dictionary_typed_key,
        PhotonObject::Dictionary(
            (0x62, 0),
            PhotonDictionary(indexmap::indexmap! {
                PhotonObject::Byte(0x00) => PhotonObject::Short(0x1234),
                PhotonObject::Byte(0x01) => PhotonObject::Byte(0xFF),
            })
        ),
        "4462000002006B12340162FF"
    );

    generate_test!(
        event_data,
        PhotonObject::EventData(EventData {
            code: 0x12,
            parameters: ParameterMap(indexmap::indexmap! {
                0x01 => PhotonObject::Short(0x1234),
                0xFF => PhotonObject::Byte(0xFF),
            })
        }),
        "65120002016B1234FF62FF"
    );

    generate_test!(
        operation_response,
        PhotonObject::OperationResponse(OperationResponse {
            operation_code: 0x12,
            return_code: -1,
            debug_message: Some("test".into()),
            parameters: ParameterMap(indexmap::indexmap! {
                0x01 => PhotonObject::Short(0x1234),
                0xFF => PhotonObject::Byte(0xFF),
            })
        }),
        "7012FFFF730004746573740002016B1234FF62FF"
    );

    generate_test!(
        operation_request,
        PhotonObject::OperationRequest(OperationRequest {
            operation_code: 0x12,
            parameters: ParameterMap(indexmap::indexmap! {
                0x01 => PhotonObject::Short(0x1234),
                0xFF => PhotonObject::Byte(0xFF),
            })
        }),
        "71120002016B1234FF62FF"
    );

    // NOTE: original code had tests to detect vec2, vec3, quaternion, etc. we're not supporting that this time
    generate_test!(
        other_custom,
        PhotonObject::Custom(CustomData::Unrecognized(15, vec![0xDE, 0xAD, 0xBE, 0xEF])),
        "630F0004DEADBEEF"
    );
}
