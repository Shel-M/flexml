use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Ident, Type, TypePath};

use crate::{conv_case, type_is_vec, DeriveAttributes, XMLAttributes};

#[derive(Default)]
pub(crate) struct StructHandler {
    pub attribute_fields: Vec<TokenStream>,
    pub node_fields: Vec<TokenStream>,
}

impl StructHandler {
    pub(crate) fn expand_tokens(
        data_struct: &DataStruct,
        xml_attributes: &XMLAttributes,
    ) -> TokenStream {
        let mut xml_field_attributes = StructHandler::process_fields(data_struct, xml_attributes);

        let mut attr_tokens = Vec::new();
        let mut node_tokens = Vec::new();

        attr_tokens.append(&mut xml_field_attributes.attribute_fields);
        node_tokens.append(&mut xml_field_attributes.node_fields);

        let node_tag = xml_attributes.get_node_tag();
        let node_ns_token = &xml_attributes.namespace_token;
        quote! {
            flexml::XML::new(#node_tag)
                #(#attr_tokens)*
                #node_ns_token
                #(#node_tokens)*
        }
    }
    pub(crate) fn process_fields(data_struct: &DataStruct, xml_attributes: &XMLAttributes) -> Self {
        let mut field_token_streams = Self::default();

        if let (Some(unit_repr), true) = (&xml_attributes.unit_repr, data_struct.fields.is_empty())
        {
            field_token_streams.node_fields.push(quote! {
                .datum(#unit_repr)
            });

            return field_token_streams;
        }

        for xml_field in data_struct.fields.iter() {
            let name = &xml_field.ident;

            let mut struct_field = StructField::from(DeriveAttributes::from(&xml_field.attrs));
            if struct_field.unserialized {
                continue;
            }
            if struct_field.case.is_none() {
                struct_field.case = xml_attributes.case_all.clone();
            }

            struct_field.name = xml_field
                .ident
                .clone()
                .expect("Expected named field")
                .to_string();

            if let Type::Path(path) = xml_field.ty.clone() {
                struct_field.ty = Some(path);
            };

            if struct_field.attribute {
                let field_str = if struct_field.alias.is_some() {
                    struct_field.alias.unwrap()
                } else if struct_field.case.is_some() {
                    conv_case(&struct_field.name, struct_field.case.unwrap())
                } else {
                    struct_field.name.clone()
                };
                field_token_streams.attribute_fields.push(quote! {
                    .attribute(#field_str, self. #name .to_string())
                })
            } else {
                let node_case = if let Some(case) = struct_field.case {
                    quote! {
                        .case(#case)
                    }
                } else {
                    quote! {}
                };

                let namespace_stream = struct_field.namespace.map(|ns| {
                    quote! {
                        .namespace(#ns).expect("Failed to set node namespace.")
                    }
                });
                let cast_stream = struct_field
                    .with
                    .map_or(quote! {.to_xml()}, |with| quote! {.#with()});

                let xml_type = &struct_field.ty.unwrap_or_else(|| {
                    panic!("Could not determine type of field {}", struct_field.name)
                });

                let stream = if type_is_vec(xml_type) {
                    quote! {
                        .data(
                            self.#name.iter()
                                .map(|d| d #cast_stream #node_case #namespace_stream)
                                .collect::<Vec<flexml::XML>>().as_slice()
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

struct StructField {
    alias: Option<String>,
    attribute: bool,
    case: Option<String>,
    name: String,
    namespace: Option<String>,
    ty: Option<TypePath>,
    unserialized: bool,
    with: Option<Ident>,
}

impl From<DeriveAttributes> for StructField {
    fn from(value: DeriveAttributes) -> Self {
        StructField {
            alias: value.alias,
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
