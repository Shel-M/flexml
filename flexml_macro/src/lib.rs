mod derive_attributes;
mod xml_enum;
mod xml_struct;

extern crate proc_macro;

use std::fmt::Display;

use derive_attributes::DeriveAttributes;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTrainCase,
    ToUpperCamelCase,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, TypePath};
use syn::{parse_macro_input, DeriveInput, Ident, LitStr, Token};
use xml_enum::EnumVariant;
use xml_struct::XMLStructFieldAttributes;

#[derive(Debug)]
struct NamespaceTuple {
    ns: LitStr,
    uri: LitStr,
}

impl Parse for NamespaceTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let ns: LitStr = content.parse()?;
        let _comma: Token![,] = content.parse()?;
        let uri: LitStr = content.parse()?;
        Ok(NamespaceTuple { ns, uri })
    }
}

#[proc_macro_derive(
    XMLNode,
    attributes(
        attribute,
        case,
        name,
        namespace,
        namespaces,
        with,
        unserialized,
        untagged
    )
)]
pub fn xml_node_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let xml_attributes = XMLAttributes::process_xml_attributes(&input);

    let ns_tokens = &xml_attributes.namespaces_tokens;
    let node_tag = &xml_attributes.name;
    let node_ns_token = &xml_attributes.namespace_tokens;

    let mut attr_tokens = Vec::new();
    let mut node_tokens = Vec::new();

    proc_macro::TokenStream::from(match &input.data {
        syn::Data::Struct(s) => {
            let mut xml_field_attributes =
                XMLStructFieldAttributes::process_field_attributes(s, &xml_attributes.with);

            attr_tokens.append(&mut xml_field_attributes.attribute_fields);
            node_tokens.append(&mut xml_field_attributes.node_fields);

            quote! {
                impl flexml::IntoXMLNode for #name {
                    fn to_xml(&self) -> flexml::XMLNode {
                        use flexml::ToXMLData;
                        // Insert the provided namespaces.
                        #(#ns_tokens)*

                        // Create the XMLNode, adding attributes and child nodes.
                        let node = flexml::XMLNode::new(#node_tag)
                            #(#attr_tokens)*
                            #node_ns_token
                            #(#node_tokens)*;
                        node
                    }

                }
            }
        }
        syn::Data::Enum(e) => {
            let xml_enum_variants = EnumVariant::process_fields(e, &xml_attributes.with);

            panic!("{:?}", xml_enum_variants);
        }
        _ => panic!("Not implemented"),
    })
}

#[derive(Default)]
struct XMLAttributes {
    name: String,
    namespace_tokens: TokenStream,
    namespaces_tokens: Vec<TokenStream>,
    with: Option<Ident>,
}

impl XMLAttributes {
    fn process_xml_attributes(input: &DeriveInput) -> Self {
        let mut ret = Self {
            name: input.ident.to_string(),
            ..Default::default()
        };

        let attrs = DeriveAttributes::from_vec(&input.attrs);

        for attr in attrs {
            match attr {
                DeriveAttributes::Name(name) => ret.name = name,
                DeriveAttributes::Namespace(namespace) => {
                    ret.namespace_tokens = quote! {
                        .namespace(#namespace).expect("Failed to set namespace")
                    }
                }
                DeriveAttributes::Namespaces(namespaces) => {
                    for namespace_tup in namespaces {
                        let (ns, uri) = (&namespace_tup.ns, &namespace_tup.uri);
                        ret.namespaces_tokens.push(quote! {
                        flexml::XMLNamespaces::insert(#ns, #uri).expect("failed to insert namespace");
                    })
                    }
                }
                DeriveAttributes::With(with) => ret.with = Some(with.clone()),
                _ => {}
            }
        }

        ret
    }
}

fn type_is_vec(typepath: &TypePath) -> bool {
    let segments = &typepath.path.segments;

    if let Some(last_seg) = segments.last() {
        last_seg.ident == "Vec"
    } else {
        false
    }
}

fn conv_case<T: Display, V: Display>(input: T, case: V) -> String {
    let input = input.to_string();
    match case.to_string().as_str() {
        "KebabCase" | "kebab-kase" => input.to_kebab_case(),
        "LowerCamelCase" | "lowerCamelCase" => input.to_lower_camel_case(),
        "ShoutyKebabCase" | "SHOUTY-KEBAB-CASE" => input.to_shouty_kebab_case(),
        // "snek" - What chicanery, what shenanigans - and dare I say it - what tomfoolery!
        "ShoutySnakeCase" | "SHOUTY_SNAKE_CASE" | "ShoutySnekCase" | "SHOUTY_SNEK_CASE" => {
            input.to_shouty_snake_case()
        }
        "SnakeCase" | "snake_case" | "SnekCase" | "snek_case" => input.to_snake_case(),

        "TitleCase" | "Title Case" => {
            panic!("XML does not allow the 'Title Case' casing scheme.")
        }
        "TrainCase" | "Train-Case" => input.to_train_case(),
        "UpperCamelCase" | "PascalCase" => input.to_upper_camel_case(),
        r => panic!("Unknown case '{r}'"),
    }
}
