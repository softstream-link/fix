#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use fix_serde::prelude::*;
use fix_serde::unittest::setup;
use serde::{Deserialize, Serialize};
use log::info;

fn test_msg_deserialize() {
    setup::log::configure();
    struct Account<T: FixStringLike>(T);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T: FixStringLike> _serde::Serialize for Account<T>
        where
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "Account",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T: FixStringLike> _serde::Deserialize<'de> for Account<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de, T: FixStringLike>
                where
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Account<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T: FixStringLike> _serde::de::Visitor<'de>
                for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    type Value = Account<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct Account",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: T = <T as _serde::Deserialize>::deserialize(__e)?;
                        _serde::__private::Ok(Account(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            T,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct Account with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Account(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "Account",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Account<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    struct AdvId<T: FixStringLike>(T);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T: FixStringLike> _serde::Deserialize<'de> for AdvId<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de, T: FixStringLike>
                where
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<AdvId<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T: FixStringLike> _serde::de::Visitor<'de>
                for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    type Value = AdvId<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct AdvId",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: T = <T as _serde::Deserialize>::deserialize(__e)?;
                        _serde::__private::Ok(AdvId(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            T,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct AdvId with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(AdvId(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "AdvId",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AdvId<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    struct BeginSeqNo(usize);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BeginSeqNo {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<BeginSeqNo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = BeginSeqNo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct BeginSeqNo",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: usize = <usize as _serde::Deserialize>::deserialize(
                            __e,
                        )?;
                        _serde::__private::Ok(BeginSeqNo(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct BeginSeqNo with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(BeginSeqNo(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "BeginSeqNo",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<BeginSeqNo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    

    struct Msg<T: FixStringLike> {
        #[serde(rename = "1")]
        #[serde(alias = "Account")]
        account: Account<T>,
        #[serde(rename = "2")]
        #[serde(alias = "AdvId")]
        adv_id: AdvId<T>,
        #[serde(rename = "7")]
        #[serde(alias = "BeginSeqNo")]
        begin_seq_no: BeginSeqNo,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T: FixStringLike> _serde::Serialize for Msg<T>
        where
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Msg",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "1",
                    &self.account,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "2",
                    &self.adv_id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "7",
                    &self.begin_seq_no,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T: FixStringLike> _serde::Deserialize<'de> for Msg<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "1" | "Account" => _serde::__private::Ok(__Field::__field0),
                            "2" | "AdvId" => _serde::__private::Ok(__Field::__field1),
                            "7" | "BeginSeqNo" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"1" | b"Account" => _serde::__private::Ok(__Field::__field0),
                            b"2" | b"AdvId" => _serde::__private::Ok(__Field::__field1),
                            b"7" | b"BeginSeqNo" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, T: FixStringLike>
                where
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Msg<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T: FixStringLike> _serde::de::Visitor<'de> for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    type Value = Msg<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Msg",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Account<T>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Msg with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            AdvId<T>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Msg with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            BeginSeqNo,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Msg with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Msg {
                            account: __field0,
                            adv_id: __field1,
                            begin_seq_no: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Account<T>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<AdvId<T>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<BeginSeqNo> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("1"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Account<T>>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("2"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<AdvId<T>>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("7"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<BeginSeqNo>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("1")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("2")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("7")?
                            }
                        };
                        _serde::__private::Ok(Msg {
                            account: __field0,
                            adv_id: __field1,
                            begin_seq_no: __field2,
                        })
                    }
                }
                
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "1",
                    "Account",
                    "2",
                    "AdvId",
                    "7",
                    "BeginSeqNo",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Msg",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Msg<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    
    let account = Account(b"ACC".try_into().unwrap());
    let adv_id = AdvId(b"ADB".try_into().unwrap());
    let begin_seq_no = BeginSeqNo(100);
    let inp_msg = Msg::<FixString> {
        account,
        adv_id,
        begin_seq_no,
    };
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("inp_msg: {0:?}", inp_msg),
                lvl,
                &("fix_serde_test", "fix_serde_test", "serde/tests/fix_serde_test.rs"),
                77u32,
                ::log::__private_api::Option::None,
            );
        }
    };
    let fix_ser = to_bytes(&inp_msg).unwrap();
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("fix_ser: {0}", fix_ser),
                lvl,
                &("fix_serde_test", "fix_serde_test", "serde/tests/fix_serde_test.rs"),
                80u32,
                ::log::__private_api::Option::None,
            );
        }
    };
    let json_ser = serde_json::to_string(&inp_msg).unwrap();
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("json_ser: {0}", json_ser),
                lvl,
                &("fix_serde_test", "fix_serde_test", "serde/tests/fix_serde_test.rs"),
                82u32,
                ::log::__private_api::Option::None,
            );
        }
    };
    let out_msg_fix: Msg<FixString> = from_slice(&fix_ser).unwrap();
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("out_msg_fix: {0:?}", out_msg_fix),
                lvl,
                &("fix_serde_test", "fix_serde_test", "serde/tests/fix_serde_test.rs"),
                84u32,
                ::log::__private_api::Option::None,
            );
        }
    };
    let out_msg_json: Msg<FixString> = serde_json::from_str(&json_ser).unwrap();
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("out_msg_json: {0:?}", out_msg_json),
                lvl,
                &("fix_serde_test", "fix_serde_test", "serde/tests/fix_serde_test.rs"),
                86u32,
                ::log::__private_api::Option::None,
            );
        }
    };
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&test_fix_string_deserialize, &test_msg_deserialize, &test_usize_deserialize],
    )
}
