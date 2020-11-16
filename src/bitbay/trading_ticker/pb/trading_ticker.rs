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
//! Generated file from `trading_ticker.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_0;

#[derive(PartialEq,Clone,Default)]
pub struct TradingTickAll {
    // message fields
    pub ticks: ::protobuf::RepeatedField<TradingTickAll_TradingTick>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TradingTickAll {
    fn default() -> &'a TradingTickAll {
        <TradingTickAll as ::protobuf::Message>::default_instance()
    }
}

impl TradingTickAll {
    pub fn new() -> TradingTickAll {
        ::std::default::Default::default()
    }

    // repeated .TradingTickAll.TradingTick ticks = 1;


    pub fn get_ticks(&self) -> &[TradingTickAll_TradingTick] {
        &self.ticks
    }
    pub fn clear_ticks(&mut self) {
        self.ticks.clear();
    }

    // Param is passed by value, moved
    pub fn set_ticks(&mut self, v: ::protobuf::RepeatedField<TradingTickAll_TradingTick>) {
        self.ticks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ticks(&mut self) -> &mut ::protobuf::RepeatedField<TradingTickAll_TradingTick> {
        &mut self.ticks
    }

    // Take field
    pub fn take_ticks(&mut self) -> ::protobuf::RepeatedField<TradingTickAll_TradingTick> {
        ::std::mem::replace(&mut self.ticks, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for TradingTickAll {
    fn is_initialized(&self) -> bool {
        for v in &self.ticks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ticks)?;
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
        for value in &self.ticks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ticks {
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

    fn new() -> TradingTickAll {
        TradingTickAll::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TradingTickAll_TradingTick>>(
                "ticks",
                |m: &TradingTickAll| { &m.ticks },
                |m: &mut TradingTickAll| { &mut m.ticks },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TradingTickAll>(
                "TradingTickAll",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TradingTickAll {
        static instance: ::protobuf::rt::LazyV2<TradingTickAll> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TradingTickAll::new)
    }
}

impl ::protobuf::Clear for TradingTickAll {
    fn clear(&mut self) {
        self.ticks.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TradingTickAll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TradingTickAll {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TradingTickAll_TradingTick {
    // message fields
    pub time: u64,
    pub lowestAsk: f32,
    pub previousRate: f32,
    pub rate: f32,
    pub highestBid: f32,
    pub scale1: u32,
    pub currency1: ::std::string::String,
    pub minOffer1: f32,
    pub scale2: u32,
    pub currency2: ::std::string::String,
    pub minOffer2: f32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TradingTickAll_TradingTick {
    fn default() -> &'a TradingTickAll_TradingTick {
        <TradingTickAll_TradingTick as ::protobuf::Message>::default_instance()
    }
}

impl TradingTickAll_TradingTick {
    pub fn new() -> TradingTickAll_TradingTick {
        ::std::default::Default::default()
    }

    // uint64 time = 1;


    pub fn get_time(&self) -> u64 {
        self.time
    }
    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u64) {
        self.time = v;
    }

    // float lowestAsk = 2;


    pub fn get_lowestAsk(&self) -> f32 {
        self.lowestAsk
    }
    pub fn clear_lowestAsk(&mut self) {
        self.lowestAsk = 0.;
    }

    // Param is passed by value, moved
    pub fn set_lowestAsk(&mut self, v: f32) {
        self.lowestAsk = v;
    }

    // float previousRate = 3;


    pub fn get_previousRate(&self) -> f32 {
        self.previousRate
    }
    pub fn clear_previousRate(&mut self) {
        self.previousRate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_previousRate(&mut self, v: f32) {
        self.previousRate = v;
    }

    // float rate = 4;


    pub fn get_rate(&self) -> f32 {
        self.rate
    }
    pub fn clear_rate(&mut self) {
        self.rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rate(&mut self, v: f32) {
        self.rate = v;
    }

    // float highestBid = 5;


    pub fn get_highestBid(&self) -> f32 {
        self.highestBid
    }
    pub fn clear_highestBid(&mut self) {
        self.highestBid = 0.;
    }

    // Param is passed by value, moved
    pub fn set_highestBid(&mut self, v: f32) {
        self.highestBid = v;
    }

    // uint32 scale1 = 6;


    pub fn get_scale1(&self) -> u32 {
        self.scale1
    }
    pub fn clear_scale1(&mut self) {
        self.scale1 = 0;
    }

    // Param is passed by value, moved
    pub fn set_scale1(&mut self, v: u32) {
        self.scale1 = v;
    }

    // string currency1 = 7;


    pub fn get_currency1(&self) -> &str {
        &self.currency1
    }
    pub fn clear_currency1(&mut self) {
        self.currency1.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency1(&mut self, v: ::std::string::String) {
        self.currency1 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency1(&mut self) -> &mut ::std::string::String {
        &mut self.currency1
    }

    // Take field
    pub fn take_currency1(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency1, ::std::string::String::new())
    }

    // float minOffer1 = 8;


    pub fn get_minOffer1(&self) -> f32 {
        self.minOffer1
    }
    pub fn clear_minOffer1(&mut self) {
        self.minOffer1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_minOffer1(&mut self, v: f32) {
        self.minOffer1 = v;
    }

    // uint32 scale2 = 9;


    pub fn get_scale2(&self) -> u32 {
        self.scale2
    }
    pub fn clear_scale2(&mut self) {
        self.scale2 = 0;
    }

    // Param is passed by value, moved
    pub fn set_scale2(&mut self, v: u32) {
        self.scale2 = v;
    }

    // string currency2 = 10;


    pub fn get_currency2(&self) -> &str {
        &self.currency2
    }
    pub fn clear_currency2(&mut self) {
        self.currency2.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency2(&mut self, v: ::std::string::String) {
        self.currency2 = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency2(&mut self) -> &mut ::std::string::String {
        &mut self.currency2
    }

    // Take field
    pub fn take_currency2(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency2, ::std::string::String::new())
    }

    // float minOffer2 = 11;


    pub fn get_minOffer2(&self) -> f32 {
        self.minOffer2
    }
    pub fn clear_minOffer2(&mut self) {
        self.minOffer2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_minOffer2(&mut self, v: f32) {
        self.minOffer2 = v;
    }
}

impl ::protobuf::Message for TradingTickAll_TradingTick {
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
                    self.time = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.lowestAsk = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.previousRate = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rate = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.highestBid = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scale1 = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency1)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.minOffer1 = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scale2 = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency2)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.minOffer2 = tmp;
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
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(1, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lowestAsk != 0. {
            my_size += 5;
        }
        if self.previousRate != 0. {
            my_size += 5;
        }
        if self.rate != 0. {
            my_size += 5;
        }
        if self.highestBid != 0. {
            my_size += 5;
        }
        if self.scale1 != 0 {
            my_size += ::protobuf::rt::value_size(6, self.scale1, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.currency1.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.currency1);
        }
        if self.minOffer1 != 0. {
            my_size += 5;
        }
        if self.scale2 != 0 {
            my_size += ::protobuf::rt::value_size(9, self.scale2, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.currency2.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.currency2);
        }
        if self.minOffer2 != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.time != 0 {
            os.write_uint64(1, self.time)?;
        }
        if self.lowestAsk != 0. {
            os.write_float(2, self.lowestAsk)?;
        }
        if self.previousRate != 0. {
            os.write_float(3, self.previousRate)?;
        }
        if self.rate != 0. {
            os.write_float(4, self.rate)?;
        }
        if self.highestBid != 0. {
            os.write_float(5, self.highestBid)?;
        }
        if self.scale1 != 0 {
            os.write_uint32(6, self.scale1)?;
        }
        if !self.currency1.is_empty() {
            os.write_string(7, &self.currency1)?;
        }
        if self.minOffer1 != 0. {
            os.write_float(8, self.minOffer1)?;
        }
        if self.scale2 != 0 {
            os.write_uint32(9, self.scale2)?;
        }
        if !self.currency2.is_empty() {
            os.write_string(10, &self.currency2)?;
        }
        if self.minOffer2 != 0. {
            os.write_float(11, self.minOffer2)?;
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

    fn new() -> TradingTickAll_TradingTick {
        TradingTickAll_TradingTick::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "time",
                |m: &TradingTickAll_TradingTick| { &m.time },
                |m: &mut TradingTickAll_TradingTick| { &mut m.time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "lowestAsk",
                |m: &TradingTickAll_TradingTick| { &m.lowestAsk },
                |m: &mut TradingTickAll_TradingTick| { &mut m.lowestAsk },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "previousRate",
                |m: &TradingTickAll_TradingTick| { &m.previousRate },
                |m: &mut TradingTickAll_TradingTick| { &mut m.previousRate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "rate",
                |m: &TradingTickAll_TradingTick| { &m.rate },
                |m: &mut TradingTickAll_TradingTick| { &mut m.rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "highestBid",
                |m: &TradingTickAll_TradingTick| { &m.highestBid },
                |m: &mut TradingTickAll_TradingTick| { &mut m.highestBid },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "scale1",
                |m: &TradingTickAll_TradingTick| { &m.scale1 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.scale1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency1",
                |m: &TradingTickAll_TradingTick| { &m.currency1 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.currency1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "minOffer1",
                |m: &TradingTickAll_TradingTick| { &m.minOffer1 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.minOffer1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "scale2",
                |m: &TradingTickAll_TradingTick| { &m.scale2 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.scale2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency2",
                |m: &TradingTickAll_TradingTick| { &m.currency2 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.currency2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "minOffer2",
                |m: &TradingTickAll_TradingTick| { &m.minOffer2 },
                |m: &mut TradingTickAll_TradingTick| { &mut m.minOffer2 },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TradingTickAll_TradingTick>(
                "TradingTickAll.TradingTick",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TradingTickAll_TradingTick {
        static instance: ::protobuf::rt::LazyV2<TradingTickAll_TradingTick> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TradingTickAll_TradingTick::new)
    }
}

impl ::protobuf::Clear for TradingTickAll_TradingTick {
    fn clear(&mut self) {
        self.time = 0;
        self.lowestAsk = 0.;
        self.previousRate = 0.;
        self.rate = 0.;
        self.highestBid = 0.;
        self.scale1 = 0;
        self.currency1.clear();
        self.minOffer1 = 0.;
        self.scale2 = 0;
        self.currency2.clear();
        self.minOffer2 = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TradingTickAll_TradingTick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TradingTickAll_TradingTick {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14trading_ticker.proto\"\xa1\x03\n\x0eTradingTickAll\x123\n\x05ticks\
    \x18\x01\x20\x03(\x0b2\x1b.TradingTickAll.TradingTickR\x05ticksB\0\x1a\
    \xd7\x02\n\x0bTradingTick\x12\x14\n\x04time\x18\x01\x20\x01(\x04R\x04tim\
    eB\0\x12\x1e\n\tlowestAsk\x18\x02\x20\x01(\x02R\tlowestAskB\0\x12$\n\x0c\
    previousRate\x18\x03\x20\x01(\x02R\x0cpreviousRateB\0\x12\x14\n\x04rate\
    \x18\x04\x20\x01(\x02R\x04rateB\0\x12\x20\n\nhighestBid\x18\x05\x20\x01(\
    \x02R\nhighestBidB\0\x12\x18\n\x06scale1\x18\x06\x20\x01(\rR\x06scale1B\
    \0\x12\x1e\n\tcurrency1\x18\x07\x20\x01(\tR\tcurrency1B\0\x12\x1e\n\tmin\
    Offer1\x18\x08\x20\x01(\x02R\tminOffer1B\0\x12\x18\n\x06scale2\x18\t\x20\
    \x01(\rR\x06scale2B\0\x12\x1e\n\tcurrency2\x18\n\x20\x01(\tR\tcurrency2B\
    \0\x12\x1e\n\tminOffer2\x18\x0b\x20\x01(\x02R\tminOffer2B\0:\0:\0B\0b\
    \x06proto3\
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
