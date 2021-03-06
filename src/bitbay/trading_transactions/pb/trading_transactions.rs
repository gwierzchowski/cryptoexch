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
//! Generated file from `trading_transactions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_0;

#[derive(PartialEq,Clone,Default)]
pub struct Transactions {
    // message fields
    pub trans: ::protobuf::RepeatedField<Transactions_Transaction>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Transactions {
    fn default() -> &'a Transactions {
        <Transactions as ::protobuf::Message>::default_instance()
    }
}

impl Transactions {
    pub fn new() -> Transactions {
        ::std::default::Default::default()
    }

    // repeated .Transactions.Transaction trans = 1;


    pub fn get_trans(&self) -> &[Transactions_Transaction] {
        &self.trans
    }
    pub fn clear_trans(&mut self) {
        self.trans.clear();
    }

    // Param is passed by value, moved
    pub fn set_trans(&mut self, v: ::protobuf::RepeatedField<Transactions_Transaction>) {
        self.trans = v;
    }

    // Mutable pointer to the field.
    pub fn mut_trans(&mut self) -> &mut ::protobuf::RepeatedField<Transactions_Transaction> {
        &mut self.trans
    }

    // Take field
    pub fn take_trans(&mut self) -> ::protobuf::RepeatedField<Transactions_Transaction> {
        ::std::mem::replace(&mut self.trans, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Transactions {
    fn is_initialized(&self) -> bool {
        for v in &self.trans {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.trans)?;
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
        for value in &self.trans {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.trans {
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

    fn new() -> Transactions {
        Transactions::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transactions_Transaction>>(
                "trans",
                |m: &Transactions| { &m.trans },
                |m: &mut Transactions| { &mut m.trans },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Transactions>(
                "Transactions",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Transactions {
        static instance: ::protobuf::rt::LazyV2<Transactions> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Transactions::new)
    }
}

impl ::protobuf::Clear for Transactions {
    fn clear(&mut self) {
        self.trans.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transactions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transactions {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Transactions_Transaction {
    // message fields
    pub timestamp: u64,
    pub ttype: Transactions_Transaction_TranType,
    pub amt: f32,
    pub rate: f32,
    pub id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Transactions_Transaction {
    fn default() -> &'a Transactions_Transaction {
        <Transactions_Transaction as ::protobuf::Message>::default_instance()
    }
}

impl Transactions_Transaction {
    pub fn new() -> Transactions_Transaction {
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

    // .Transactions.Transaction.TranType ttype = 2;


    pub fn get_ttype(&self) -> Transactions_Transaction_TranType {
        self.ttype
    }
    pub fn clear_ttype(&mut self) {
        self.ttype = Transactions_Transaction_TranType::BYE;
    }

    // Param is passed by value, moved
    pub fn set_ttype(&mut self, v: Transactions_Transaction_TranType) {
        self.ttype = v;
    }

    // float amt = 3;


    pub fn get_amt(&self) -> f32 {
        self.amt
    }
    pub fn clear_amt(&mut self) {
        self.amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_amt(&mut self, v: f32) {
        self.amt = v;
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

    // string id = 5;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Transactions_Transaction {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.ttype, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.amt = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rate = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if self.ttype != Transactions_Transaction_TranType::BYE {
            my_size += ::protobuf::rt::enum_size(2, self.ttype);
        }
        if self.amt != 0. {
            my_size += 5;
        }
        if self.rate != 0. {
            my_size += 5;
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.timestamp != 0 {
            os.write_uint64(1, self.timestamp)?;
        }
        if self.ttype != Transactions_Transaction_TranType::BYE {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.ttype))?;
        }
        if self.amt != 0. {
            os.write_float(3, self.amt)?;
        }
        if self.rate != 0. {
            os.write_float(4, self.rate)?;
        }
        if !self.id.is_empty() {
            os.write_string(5, &self.id)?;
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

    fn new() -> Transactions_Transaction {
        Transactions_Transaction::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "timestamp",
                |m: &Transactions_Transaction| { &m.timestamp },
                |m: &mut Transactions_Transaction| { &mut m.timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Transactions_Transaction_TranType>>(
                "ttype",
                |m: &Transactions_Transaction| { &m.ttype },
                |m: &mut Transactions_Transaction| { &mut m.ttype },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "amt",
                |m: &Transactions_Transaction| { &m.amt },
                |m: &mut Transactions_Transaction| { &mut m.amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "rate",
                |m: &Transactions_Transaction| { &m.rate },
                |m: &mut Transactions_Transaction| { &mut m.rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Transactions_Transaction| { &m.id },
                |m: &mut Transactions_Transaction| { &mut m.id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Transactions_Transaction>(
                "Transactions.Transaction",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Transactions_Transaction {
        static instance: ::protobuf::rt::LazyV2<Transactions_Transaction> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Transactions_Transaction::new)
    }
}

impl ::protobuf::Clear for Transactions_Transaction {
    fn clear(&mut self) {
        self.timestamp = 0;
        self.ttype = Transactions_Transaction_TranType::BYE;
        self.amt = 0.;
        self.rate = 0.;
        self.id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transactions_Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transactions_Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Transactions_Transaction_TranType {
    BYE = 0,
    SELL = 1,
}

impl ::protobuf::ProtobufEnum for Transactions_Transaction_TranType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Transactions_Transaction_TranType> {
        match value {
            0 => ::std::option::Option::Some(Transactions_Transaction_TranType::BYE),
            1 => ::std::option::Option::Some(Transactions_Transaction_TranType::SELL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Transactions_Transaction_TranType] = &[
            Transactions_Transaction_TranType::BYE,
            Transactions_Transaction_TranType::SELL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Transactions_Transaction_TranType>("Transactions.Transaction.TranType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Transactions_Transaction_TranType {
}

impl ::std::default::Default for Transactions_Transaction_TranType {
    fn default() -> Self {
        Transactions_Transaction_TranType::BYE
    }
}

impl ::protobuf::reflect::ProtobufValue for Transactions_Transaction_TranType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1atrading_transactions.proto\"\x8e\x02\n\x0cTransactions\x121\n\x05t\
    rans\x18\x01\x20\x03(\x0b2\x19.Transactions.TransactionR\x05transB\0\x1a\
    \xc8\x01\n\x0bTransaction\x12\x1e\n\ttimestamp\x18\x01\x20\x01(\x04R\tti\
    mestampB\0\x12:\n\x05ttype\x18\x02\x20\x01(\x0e2\".Transactions.Transact\
    ion.TranTypeR\x05ttypeB\0\x12\x12\n\x03amt\x18\x03\x20\x01(\x02R\x03amtB\
    \0\x12\x14\n\x04rate\x18\x04\x20\x01(\x02R\x04rateB\0\x12\x10\n\x02id\
    \x18\x05\x20\x01(\tR\x02idB\0\"\x1f\n\x08TranType\x12\x07\n\x03BYE\x10\0\
    \x12\x08\n\x04SELL\x10\x01\x1a\0:\0:\0B\0b\x06proto3\
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
