use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, Ident, LitStr, Type, TypePath};

use crate::{conv_case, type_is_vec, DeriveAttributes};

#[derive(Default)]
pub(crate) struct XMLStructFieldAttributes {
    pub attribute_fields: Vec<TokenStream>,
    pub node_fields: Vec<TokenStream>,
}

impl XMLStructFieldAttributes {
    pub(crate) fn process_field_attributes(data_struct: &DataStruct, with: &Option<Ident>) -> Self {
        let mut field_token_streams = Self::default();

        for field in data_struct.fields.iter() {
            let name = &field.ident;

            let mut xml_field = XMLField::from(DeriveAttributes::from(&field.attrs));
            if xml_field.unserialized {
                continue;
            }

            xml_field.name = field
                .ident
                .clone()
                .expect("Expected named field")
                .to_string();
            if xml_field.with.is_none() {
                xml_field.with = with.clone();
            };

            if let Type::Path(path) = field.ty.clone() {
                xml_field.ty = Some(path);
            };

            if xml_field.attribute {
                let field_str = if xml_field.case.is_some() {
                    conv_case(&xml_field.name, xml_field.case.unwrap())
                } else {
                    xml_field.name.clone()
                };
                field_token_streams.attribute_fields.push(quote! {
                    .attribute(#field_str, self. #name .to_string())
                })
            } else {
                let node_case = if let Some(case) = xml_field.case {
                    quote! {
                        .case(#case).expect("Could not set node case.")
                    }
                } else {
                    quote! {}
                };

                let namespace_stream = xml_field.namespace.map_or(quote! {}, |ns| {
                    quote! {
                        .namespace(#ns).expect("Failed to set node namespace.")
                    }
                });
                let cast_stream = xml_field
                    .with
                    .map_or(quote! {.to_xml_data()}, |with| quote! {.#with()});

                let xml_type = &xml_field.ty.unwrap_or_else(|| {
                    panic!("Could not determine type of field {}", xml_field.name)
                });

                let stream = if type_is_vec(xml_type) {
                    quote! {
                        .data(
                            self.#name.iter()
                                .map(|d| d #cast_stream #node_case #namespace_stream)
                                .collect::<Vec<flexml::XMLData>>().as_slice()
                        )
                    }
                } else {
                    quote! {
                        .datum(self.#name #cast_stream #node_case #namespace_stream)
                    }
                };

                field_token_streams.node_fields.push(stream);
            }
        }

        field_token_streams
    }
}

struct XMLField {
    attribute: bool,
    case: Option<String>,
    name: String,
    namespace: Option<String>,
    ty: Option<TypePath>,
    unserialized: bool,
    with: Option<Ident>,
}

impl From<DeriveAttributes> for XMLField {
    fn from(value: DeriveAttributes) -> Self {
        XMLField {
            attribute: value.attribute,
            case: value.case,
            name: String::new(),
            namespace: value.namespace,
            ty: None,
            unserialized: value.unserialized,
            with: value.with,
        }
    }
}
