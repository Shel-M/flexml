use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Ident, Type, TypePath};

use crate::{conv_case, type_is_vec, DeriveAttributes};

#[derive(Default)]
pub(crate) struct XMLStructFieldAttributes {
    pub attribute_fields: Vec<TokenStream>,
    pub node_fields: Vec<TokenStream>,
}

impl XMLStructFieldAttributes {
    pub(crate) fn process_field_attributes(data_struct: &DataStruct, with: &Option<Ident>) -> Self {
        let mut field_token_streams = Self::default();

        struct XMLField {
            attribute: bool,
            name: Ident,
            case: Option<String>,
            ty: Option<TypePath>,
            namespace: Option<String>,
            with: Option<Ident>,
        }

        'field: for field in data_struct.fields.iter() {
            let field_ident = field.ident.as_ref().expect("Expected named field");
            let mut xml_field: XMLField = XMLField {
                attribute: false,
                name: field_ident.clone(),
                case: None,
                ty: None,
                namespace: None,
                with: with.clone(),
            };

            if let Type::Path(path) = field.ty.clone() {
                xml_field.ty = Some(path);
            };

            for attr in DeriveAttributes::from_vec(&field.attrs) {
                match attr {
                    DeriveAttributes::Attribute => xml_field.attribute = true,
                    DeriveAttributes::Case(c) => {
                        xml_field.case = Some(c);
                    }
                    DeriveAttributes::Namespace(ns) => xml_field.namespace = Some(ns),
                    DeriveAttributes::With(with) => xml_field.with = Some(with),
                    DeriveAttributes::Unserialized => continue 'field,
                    _ => {}
                }
            }

            if xml_field.attribute {
                let field_str = if xml_field.case.is_some() {
                    conv_case(&xml_field.name, xml_field.case.unwrap())
                } else {
                    xml_field.name.to_string()
                };
                field_token_streams.attribute_fields.push(quote! {
                    .attribute(#field_str, format!("{}", self.#field_ident))
                })
            } else {
                let node_name = xml_field.name;
                let node_case = if let Some(case) = xml_field.case {
                    quote! {
                        .case(#case).expect("Could not set node case.")
                    }
                } else {
                    quote! {}
                };

                let namespace_stream = match xml_field.namespace {
                    Some(ns) => quote! {
                        .namespace(#ns).expect("Failed to set node namespace.")
                    },
                    None => quote! {},
                };
                let cast_stream = if let Some(with) = &xml_field.with {
                    quote! {.#with()}
                } else {
                    quote! {.to_xml_data()}
                };

                let stream = if let Some(ty) = xml_field.ty {
                    if type_is_vec(&ty) {
                        quote! {
                            .data(
                                self.#node_name.iter()
                                    .map(|d| d #cast_stream #node_case #namespace_stream)
                                    .collect::<Vec<flexml::XMLData>>().as_slice()
                            )
                        }
                    } else {
                        quote! {
                            .datum(self.#node_name #cast_stream #node_case #namespace_stream)
                        }
                    }
                } else {
                    panic!("Could not determine type of field {node_name}");
                };

                field_token_streams.node_fields.push(stream);
            }
        }

        field_token_streams
    }
}
