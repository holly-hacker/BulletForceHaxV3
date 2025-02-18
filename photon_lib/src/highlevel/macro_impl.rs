// TODO: allow opting out of custom_properties

// dev note: you probably want to read https://danielkeep.github.io/tlborm/book/
// This is a hacky macro, macro_rules is probably not intended for some of the things I'm doing (like req/opt).
// In fact, some of the syntax is defined by the limitations of macro_rules.

macro_rules! impl_u8_map_conversion {
    (
        $(
            $(#[$type_attr:meta])*
            $type_name:ident {
                $(
                    $(#[$field_attr:meta])*
                    $(
                        @required
                        [$map_key_req:expr $(=> $map_type_req:path)?]
                        $field_name_req:ident: $field_type_req:ty
                    )?
                    $(
                        [$map_key_opt:expr $(=> $map_type_opt:path)?]
                        $field_name_opt:ident: $field_type_opt:ty
                    )?
                    ,
                )*
            }
        )*
    ) => {
        $(
            $(#[$type_attr])*
            pub struct $type_name {
                $(
                    $(#[$field_attr])*
                    $(pub $field_name_opt: Option<$field_type_opt>,)?
                    $(pub $field_name_req: $field_type_req,)?
                )*
            }

            impl std::convert::TryFrom<crate::ParameterMap> for $type_name {
                type Error = crate::highlevel::LiftingError;

                fn try_from(mut properties: crate::ParameterMap) -> Result<Self, crate::highlevel::LiftingError> {
                    Ok($type_name {
                        // NOTE: we need to use `shift_remove` to retain order for custom_properties later
                        // this may not actually be important, but it allows types converted both ways and be identical
                        $(
                            $(
                                $field_name_req: match properties.0.shift_remove(&$map_key_req) {
                                    #[allow(unused_parens)]
                                    Some($($map_type_req)?(b)) => b.try_into()?,
                                    #[allow(unreachable_patterns)]
                                    Some(k) => {
                                        return Err(crate::highlevel::LiftingError::UnexpectedObjectTypeInStruct {
                                            struct_name: stringify!($type_name),
                                            field_name: stringify!($field_name_req),
                                            expected_type: stringify!($($map_type_req)?),
                                            actual_value: Box::new(k),
                                        });
                                    }
                                    None => {
                                        return Err(crate::highlevel::LiftingError::MissingRequiredField {
                                            struct_name: stringify!($type_name),
                                            field_name: stringify!($field_name_req),
                                        });
                                    }
                                },
                            )?
                            $(
                                $field_name_opt: match properties.0.shift_remove(&$map_key_opt) {
                                    #[allow(unused_parens)]
                                    Some($($map_type_opt)?(b)) => Some(b.try_into()?),
                                    #[allow(unreachable_patterns)]
                                    Some(k) => {
                                        return Err(crate::highlevel::LiftingError::UnexpectedObjectTypeInStruct {
                                            struct_name: stringify!($type_name),
                                            field_name: stringify!($field_name_opt),
                                            expected_type: stringify!($($map_type_opt)?),
                                            actual_value: Box::new(k),
                                        });
                                    }
                                    _ => None,
                                },
                            )?
                        )*
                    })
                }
            }

            impl std::convert::From<$type_name> for crate::ParameterMap {
                fn from(#[allow(unused_mut)] mut value: $type_name) -> Self {
                    let mut map = Self::default();

                    $(
                        $(
                            map.0.insert($map_key_req, $($map_type_req)?(value.$field_name_req.into()));
                        )?
                        $(
                            if let Some(b) = value.$field_name_opt.take() {
                                map.0.insert($map_key_opt, $($map_type_opt)?(b.into()));
                            }
                        )?
                    )*

                    map
                }
            }
        )*
    };
}

macro_rules! impl_photon_map_conversion {
    (
        $(
            $(#[$type_attr:meta])*
            $type_name:ident {
                $(
                    $(#[$field_attr:meta])*
                    $(
                        @required
                        [$map_key_req:expr $(=> $map_type_req:path)?]
                        $field_name_req:ident: $field_type_req:ty
                    )?
                    $(
                        [$map_key_opt:expr $(=> $map_type_opt:path)?]
                        $field_name_opt:ident: $field_type_opt:ty
                    )?
                    ,
                )*
            }
        )*
    ) => {
        $(
            $(#[$type_attr])*
            pub struct $type_name {
                $(
                    $(#[$field_attr])*
                    $(pub $field_name_opt: Option<$field_type_opt>,)?
                    $(pub $field_name_req: $field_type_req,)?
                )*

                pub custom_properties: indexmap::IndexMap<String, PhotonObject>,
            }

            impl std::convert::TryFrom<crate::PhotonHashmap> for $type_name {
                type Error = crate::highlevel::LiftingError;

                fn try_from(mut properties: crate::PhotonHashmap) -> Result<Self, crate::highlevel::LiftingError> {
                    Ok($type_name {
                        $(
                            // NOTE: we need to use `shift_remove` to retain order for custom_properties later
                            // this may not actually be important, but it allows types converted both ways and be identical
                            $(
                                $field_name_req: match properties.0.shift_remove(&$map_key_req) {
                                    #[allow(unused_parens)]
                                    Some($($map_type_req)?(b)) => b.try_into()?,
                                    #[allow(unreachable_patterns)]
                                    Some(k) => {
                                        let error_message = format!(
                                            "When converting {} from map, found {k:?} when expecting object type {}",
                                            stringify!($type_name), stringify!($($map_type_req)?));
                                        tracing::error!("{}", error_message);
                                        return Err(crate::highlevel::LiftingError::UnexpectedObjectTypeInStruct {
                                            struct_name: stringify!($type_name),
                                            field_name: stringify!($field_name_req),
                                            expected_type: stringify!($($map_type_req)?),
                                            actual_value: Box::new(k),
                                        });
                                    }
                                    _ => todo!("error handling in from_map for missing req field"), // TODO: error handling here!!
                                },
                            )?
                            $(
                                $field_name_opt: match properties.0.shift_remove(&$map_key_opt) {
                                    #[allow(unused_parens)]
                                    Some($($map_type_opt)?(b)) => Some(b.try_into()?),
                                    #[allow(unreachable_patterns)]
                                    Some(k) => {
                                        return Err(crate::highlevel::LiftingError::UnexpectedObjectTypeInStruct {
                                            struct_name: stringify!($type_name),
                                            field_name: stringify!($field_name_opt),
                                            expected_type: stringify!($($map_type_opt)?),
                                            actual_value: Box::new(k),
                                        });
                                    }
                                    _ => None,
                                },
                            )?
                        )*

                        custom_properties: properties.0
                            .drain(..)
                            .filter_map(|(k, v)| match k {
                                PhotonObject::String(k) => Some((k, v)),
                                k => {
                                    tracing::warn!(
                                        "When mapping remaining props to custom props for {} from map, found unmapped, non-string key {k:?}",
                                        stringify!($type_name));
                                    None
                                }
                            })
                            .collect::<indexmap::IndexMap<String, PhotonObject>>(),
                    })
                }
            }

            impl std::convert::TryFrom<crate::PhotonObject> for $type_name {
                type Error = crate::highlevel::LiftingError;

                fn try_from(value: crate::PhotonObject) -> Result<Self, crate::highlevel::LiftingError> {
                    if let crate::PhotonObject::Hashtable(properties) = value {
                        Ok(properties.try_into()?)
                    } else {
                        Err(crate::highlevel::WrongPhotonObjectError {
                            // NOTE: type name is known, add this?
                            expected_type: stringify!(Hashtable),
                            actual_value: Box::new(value),
                        }.into())
                    }
                }
            }

            impl std::convert::From<$type_name> for crate::PhotonHashmap {
                fn from(#[allow(unused_mut)] mut value: $type_name) -> Self {
                    let mut map = Self::default();

                    $(
                        $(
                            map.0.insert($map_key_req, $($map_type_req)?(value.$field_name_req.into()));
                        )?
                        $(
                            if let Some(b) = value.$field_name_opt.take() {
                                map.0.insert($map_key_opt, $($map_type_opt)?(b.into()));
                            }
                        )?
                    )*

                    for (k, v) in value.custom_properties.drain(..) {
                        map.0.insert(PhotonObject::String(k), v);
                    }

                    map
                }
            }

            impl std::convert::From<$type_name> for crate::PhotonObject {
                fn from(#[allow(unused_mut)] mut value: $type_name) -> Self {
                    Self::Hashtable(value.into())
                }
            }
        )*
    };
}
