// This file is generated by rust-protobuf 2.18.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `trading_stats.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_0;

#[derive(PartialEq,Clone,Default)]
pub struct TradingStatsAll {
    // message fields
    pub stats: ::protobuf::RepeatedField<TradingStatsAll_TradingStat>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TradingStatsAll {
    fn default() -> &'a TradingStatsAll {
        <TradingStatsAll as ::protobuf::Message>::default_instance()
    }
}

impl TradingStatsAll {
    pub fn new() -> TradingStatsAll {
        ::std::default::Default::default()
    }

    // repeated .TradingStatsAll.TradingStat stats = 1;


    pub fn get_stats(&self) -> &[TradingStatsAll_TradingStat] {
        &self.stats
    }
    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: ::protobuf::RepeatedField<TradingStatsAll_TradingStat>) {
        self.stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats(&mut self) -> &mut ::protobuf::RepeatedField<TradingStatsAll_TradingStat> {
        &mut self.stats
    }

    // Take field
    pub fn take_stats(&mut self) -> ::protobuf::RepeatedField<TradingStatsAll_TradingStat> {
        ::std::mem::replace(&mut self.stats, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TradingStatsAll {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.stats {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TradingStatsAll {
        TradingStatsAll::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TradingStatsAll_TradingStat>>(
                "stats",
                |m: &TradingStatsAll| { &m.stats },
                |m: &mut TradingStatsAll| { &mut m.stats },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TradingStatsAll>(
                "TradingStatsAll",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TradingStatsAll {
        static instance: ::protobuf::rt::LazyV2<TradingStatsAll> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TradingStatsAll::new)
    }
}

impl ::protobuf::Clear for TradingStatsAll {
    fn clear(&mut self) {
        self.stats.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TradingStatsAll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TradingStatsAll {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TradingStatsAll_TradingStat {
    // message fields
    pub timestamp: u64,
    pub market1: ::std::string::String,
    pub market2: ::std::string::String,
    pub vol: f32,
    pub hi: f32,
    pub lo: f32,
    pub r24h: f32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TradingStatsAll_TradingStat {
    fn default() -> &'a TradingStatsAll_TradingStat {
        <TradingStatsAll_TradingStat as ::protobuf::Message>::default_instance()
    }
}

impl TradingStatsAll_TradingStat {
    pub fn new() -> TradingStatsAll_TradingStat {
        ::std::default::Default::default()
    }

    // uint64 timestamp = 1;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    // string market1 = 2;


    pub fn get_market1(&self) -> &str {
        &self.market1
    }
    pub fn clear_market1(&mut self) {
        self.market1.clear();
    }

    // Param is passed by value, moved
    pub fn set_market1(&mut self, v: ::std::string::String) {
        self.market1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_market1(&mut self) -> &mut ::std::string::String {
        &mut self.market1
    }

    // Take field
    pub fn take_market1(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.market1, ::std::string::String::new())
    }

    // string market2 = 3;


    pub fn get_market2(&self) -> &str {
        &self.market2
    }
    pub fn clear_market2(&mut self) {
        self.market2.clear();
    }

    // Param is passed by value, moved
    pub fn set_market2(&mut self, v: ::std::string::String) {
        self.market2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_market2(&mut self) -> &mut ::std::string::String {
        &mut self.market2
    }

    // Take field
    pub fn take_market2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.market2, ::std::string::String::new())
    }

    // float vol = 5;


    pub fn get_vol(&self) -> f32 {
        self.vol
    }
    pub fn clear_vol(&mut self) {
        self.vol = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vol(&mut self, v: f32) {
        self.vol = v;
    }

    // float hi = 7;


    pub fn get_hi(&self) -> f32 {
        self.hi
    }
    pub fn clear_hi(&mut self) {
        self.hi = 0.;
    }

    // Param is passed by value, moved
    pub fn set_hi(&mut self, v: f32) {
        self.hi = v;
    }

    // float lo = 8;


    pub fn get_lo(&self) -> f32 {
        self.lo
    }
    pub fn clear_lo(&mut self) {
        self.lo = 0.;
    }

    // Param is passed by value, moved
    pub fn set_lo(&mut self, v: f32) {
        self.lo = v;
    }

    // float r24h = 9;


    pub fn get_r24h(&self) -> f32 {
        self.r24h
    }
    pub fn clear_r24h(&mut self) {
        self.r24h = 0.;
    }

    // Param is passed by value, moved
    pub fn set_r24h(&mut self, v: f32) {
        self.r24h = v;
    }
}

impl ::protobuf::Message for TradingStatsAll_TradingStat {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.market1)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.market2)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.vol = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.hi = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.lo = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.r24h = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(1, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.market1.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.market1);
        }
        if !self.market2.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.market2);
        }
        if self.vol != 0. {
            my_size += 5;
        }
        if self.hi != 0. {
            my_size += 5;
        }
        if self.lo != 0. {
            my_size += 5;
        }
        if self.r24h != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.timestamp != 0 {
            os.write_uint64(1, self.timestamp)?;
        }
        if !self.market1.is_empty() {
            os.write_string(2, &self.market1)?;
        }
        if !self.market2.is_empty() {
            os.write_string(3, &self.market2)?;
        }
        if self.vol != 0. {
            os.write_float(5, self.vol)?;
        }
        if self.hi != 0. {
            os.write_float(7, self.hi)?;
        }
        if self.lo != 0. {
            os.write_float(8, self.lo)?;
        }
        if self.r24h != 0. {
            os.write_float(9, self.r24h)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TradingStatsAll_TradingStat {
        TradingStatsAll_TradingStat::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "timestamp",
                |m: &TradingStatsAll_TradingStat| { &m.timestamp },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "market1",
                |m: &TradingStatsAll_TradingStat| { &m.market1 },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.market1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "market2",
                |m: &TradingStatsAll_TradingStat| { &m.market2 },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.market2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "vol",
                |m: &TradingStatsAll_TradingStat| { &m.vol },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.vol },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "hi",
                |m: &TradingStatsAll_TradingStat| { &m.hi },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.hi },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "lo",
                |m: &TradingStatsAll_TradingStat| { &m.lo },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.lo },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "r24h",
                |m: &TradingStatsAll_TradingStat| { &m.r24h },
                |m: &mut TradingStatsAll_TradingStat| { &mut m.r24h },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TradingStatsAll_TradingStat>(
                "TradingStatsAll.TradingStat",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TradingStatsAll_TradingStat {
        static instance: ::protobuf::rt::LazyV2<TradingStatsAll_TradingStat> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TradingStatsAll_TradingStat::new)
    }
}

impl ::protobuf::Clear for TradingStatsAll_TradingStat {
    fn clear(&mut self) {
        self.timestamp = 0;
        self.market1.clear();
        self.market2.clear();
        self.vol = 0.;
        self.hi = 0.;
        self.lo = 0.;
        self.r24h = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TradingStatsAll_TradingStat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TradingStatsAll_TradingStat {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13trading_stats.proto\"\x81\x02\n\x0fTradingStatsAll\x124\n\x05stats\
    \x18\x01\x20\x03(\x0b2\x1c.TradingStatsAll.TradingStatR\x05statsB\0\x1a\
    \xb5\x01\n\x0bTradingStat\x12\x1e\n\ttimestamp\x18\x01\x20\x01(\x04R\tti\
    mestampB\0\x12\x1a\n\x07market1\x18\x02\x20\x01(\tR\x07market1B\0\x12\
    \x1a\n\x07market2\x18\x03\x20\x01(\tR\x07market2B\0\x12\x12\n\x03vol\x18\
    \x05\x20\x01(\x02R\x03volB\0\x12\x10\n\x02hi\x18\x07\x20\x01(\x02R\x02hi\
    B\0\x12\x10\n\x02lo\x18\x08\x20\x01(\x02R\x02loB\0\x12\x14\n\x04r24h\x18\
    \t\x20\x01(\x02R\x04r24hB\0:\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
