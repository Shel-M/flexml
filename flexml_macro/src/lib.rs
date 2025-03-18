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
use syn::{parenthesized, Lit, TypePath};
use syn::{parse_macro_input, DeriveInput, LitStr, Token};
use xml_enum::EnumHandler;
use xml_struct::StructHandler;

#[derive(Debug)]
enum NamespaceTuple {
    Ns { ns: LitStr, uri: LitStr },
}

impl Parse for NamespaceTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let ns: LitStr = content.parse()?;
        let _comma: Token![,] = content.parse()?;
        let uri: LitStr = content.parse()?;
        Ok(NamespaceTuple::Ns { ns, uri })
    }
}

#[proc_macro_derive(
    ToXML,
    attributes(
        attribute,
        case,
        case_all,
        name,
        namespace,
        namespaces,
        with,
        unit_repr,
        unserialized,
        untagged
    )
)]
pub fn xml_node_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let xml_attributes = XMLAttributes::process_xml_attributes(&input);

    let ns_tokens = &xml_attributes.namespaces_tokens;

    let expanded_body = match &input.data {
        syn::Data::Struct(data_struct) => {
            StructHandler::expand_tokens(data_struct, &xml_attributes)
        }
        syn::Data::Enum(data_enum) => EnumHandler::expand_tokens(data_enum, &xml_attributes),
        _ => panic!("Not implemented"),
    };
    proc_macro::TokenStream::from(quote! {
        impl flexml::IntoXML for #name {
            fn to_xml(&self) -> flexml::XML {
                #(#ns_tokens)*

                #expanded_body
            }
        }
    })
}

#[derive(Default)]
struct XMLAttributes {
    alias: Option<String>,
    case: Option<String>,
    case_all: Option<String>,
    name: String,
    namespace_token: Option<TokenStream>,
    namespaces_tokens: Vec<TokenStream>,
    unit_repr: Option<Lit>,
    untagged: bool,
}

impl XMLAttributes {
    fn process_xml_attributes(input: &DeriveInput) -> Self {
        let mut xml_attributes = Self::from(DeriveAttributes::from(&input.attrs));
        xml_attributes.name = input.ident.to_string();
        xml_attributes
    }

    fn get_node_tag(&self) -> String {
        if self.alias.is_some() {
            self.alias.as_ref().unwrap().to_string()
        } else if self.case.is_some() {
            conv_case(&self.name, self.case.as_ref().unwrap())
        } else {
            self.name.to_string()
        }
    }
}

impl From<DeriveAttributes> for XMLAttributes {
    fn from(value: DeriveAttributes) -> Self {
        if value.with.is_some() {
            if cfg!(test) {
                panic!(
                    "`with` attribute is unsupported on container types \n {:#?}",
                    value.with
                )
            }
            panic!("`with` attribute is unsupported on container types")
        }

        Self {
            alias: value.alias,
            case: value.case,
            case_all: value.case_all,
            name: String::new(),
            namespace_token: value.namespace.map(|namespace| {
                quote! {
                    .namespace(#namespace).expect("Failed to set namespace")
                }
            }),
            namespaces_tokens: value.namespaces.iter().map(|ns_tuple| {
                let NamespaceTuple::Ns{ns, uri} = ns_tuple;
                quote! {
                flexml::XMLNamespaces::insert(#ns, #uri).expect("failed to insert namespace");
            }}).collect(),
            unit_repr: value.unit_repr,
            untagged: value.untagged
        }
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
