// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `coin.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

///  Coin defines a token with a denomination and an amount.
///
///  NOTE: The amount field is an Int which implements the custom method
///  signatures required by gogoproto.
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:cosmos.base.v1beta1.Coin)
pub struct Coin {
    // message fields
    // @@protoc_insertion_point(field:cosmos.base.v1beta1.Coin.denom)
    pub denom: ::std::string::String,
    // @@protoc_insertion_point(field:cosmos.base.v1beta1.Coin.amount)
    pub amount: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:cosmos.base.v1beta1.Coin.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Coin {
    fn default() -> &'a Coin {
        <Coin as ::protobuf::Message>::default_instance()
    }
}

impl Coin {
    pub fn new() -> Coin {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "denom",
            |m: &Coin| { &m.denom },
            |m: &mut Coin| { &mut m.denom },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "amount",
            |m: &Coin| { &m.amount },
            |m: &mut Coin| { &mut m.amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Coin>(
            "Coin",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Coin {
    const NAME: &'static str = "Coin";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.denom = is.read_string()?;
                },
                18 => {
                    self.amount = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.denom);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.denom.is_empty() {
            os.write_string(1, &self.denom)?;
        }
        if !self.amount.is_empty() {
            os.write_string(2, &self.amount)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Coin {
        Coin::new()
    }

    fn clear(&mut self) {
        self.denom.clear();
        self.amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Coin {
        static instance: Coin = Coin {
            denom: ::std::string::String::new(),
            amount: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Coin {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Coin").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Coin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Coin {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ncoin.proto\x12\x13cosmos.base.v1beta1\"4\n\x04Coin\x12\x14\n\x05deno\
    m\x18\x01\x20\x01(\tR\x05denom\x12\x16\n\x06amount\x18\x02\x20\x01(\tR\
    \x06amountJ\xc8\x02\n\x06\x12\x04\0\0\x0c\x01\n\x08\n\x01\x0c\x12\x03\0\
    \0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x1c\n\xaf\x01\n\x02\x04\0\x12\x04\
    \x07\0\x0c\x01\x1a\xa2\x01\x20Coin\x20defines\x20a\x20token\x20with\x20a\
    \x20denomination\x20and\x20an\x20amount.\n\n\x20NOTE:\x20The\x20amount\
    \x20field\x20is\x20an\x20Int\x20which\x20implements\x20the\x20custom\x20\
    method\n\x20signatures\x20required\x20by\x20gogoproto.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x07\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02\x14\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\t\t\x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x12\x13\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\n\x02\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n\t\x0f\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\n\x12\x13b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Coin::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}