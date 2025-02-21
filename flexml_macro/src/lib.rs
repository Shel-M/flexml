extern crate proc_macro;

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTrainCase,
    ToUpperCamelCase,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Type;
use syn::{parenthesized, TypePath};
use syn::{parse_macro_input, DeriveInput, Ident, LitStr, Token};

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

#[proc_macro_derive(XMLNode, attributes(attribute, name, namespace, namespaces, node))]
pub fn xml_node_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let xml_struct_attributes = XMLStructAttributes::process_struct_attributes(&input);
    let xml_field_attributes = XMLFieldAttributes::process_field_attributes(&input);

    let name = input.ident;
    let ns_tokens = &xml_struct_attributes.namespaces_tokens;
    let struct_tag = &xml_struct_attributes.name;
    let node_ns_token = &xml_struct_attributes.namespace_tokens;

    let attr_tokens = &xml_field_attributes.attribute_fields;
    let node_tokens = &xml_field_attributes.node_fields;

    let expanded = quote! {
        impl flexml::IntoXMLNode for #name {
            fn to_xml(&self) -> flexml::XMLNode {
                use flexml::ToXMLData;
                // Insert the provided namespaces.
                #(#ns_tokens)*

                // Create the XMLNode, adding attributes and child nodes.
                let node = flexml::XMLNode::new(#struct_tag)
                    #(#attr_tokens)*
                    #node_ns_token
                    #(#node_tokens)*;
                node
            }

        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[derive(Default)]
struct XMLStructAttributes {
    name: String,
    namespace_tokens: TokenStream,
    namespaces_tokens: Vec<TokenStream>,
}

impl XMLStructAttributes {
    fn process_struct_attributes(input: &DeriveInput) -> Self {
        let mut ret = Self::default();
        let name = &input.ident;

        let mut xml_name = None;

        for attr in input.attrs.iter() {
            if attr.path().is_ident("name") {
                let n: LitStr = attr
                    .parse_args()
                    .expect("Expected string literal in namespace attribute");
                xml_name = Some(n.value());
            } else if attr.path().is_ident("namespaces") {
                let namespaces: Punctuated<NamespaceTuple, Token![,]> = attr
                    .parse_args_with(Punctuated::parse_terminated)
                    .expect("Failed to parse namespaces attribute");

                for namespace in namespaces {
                    let (ns, uri) = (namespace.ns.value(), namespace.uri.value());
                    ret.namespaces_tokens.push(quote! {
                        flexml::XMLNamespaces::insert(#ns, #uri).expect("failed to insert namespace");
                    });
                }
            } else if attr.path().is_ident("namespace") {
                // For the struct-level namespace, parse as a string literal.
                let ns: LitStr = attr
                    .parse_args()
                    .expect("Expected string literal in namespace attribute");
                ret.namespace_tokens = quote! {
                    .namespace(#ns).expect("Failed to set doc namespace")
                }
            }
        }

        // Use the lowercased struct name as the XML element tag.
        ret.name = if let Some(doc_name) = xml_name {
            doc_name
        } else {
            name.to_string()
        };

        ret
    }
}

#[derive(Default)]
struct XMLFieldAttributes {
    attribute_fields: Vec<TokenStream>,
    node_fields: Vec<TokenStream>,
}

impl XMLFieldAttributes {
    fn process_field_attributes(input: &DeriveInput) -> Self {
        let mut ret = Self::default();

        // Ensure that the input is a struct with named fields.
        let fields = if let syn::Data::Struct(data_struct) = &input.data {
            match &data_struct.fields {
                syn::Fields::Named(fields_named) => &fields_named.named,
                _ => panic!("XMLDoc can only be derived for structs with named fields"),
            }
        } else {
            panic!("XMLDoc can only be derived for structs");
        };

        // Prepare vectors for fields marked as attributes or nodes.
        let mut node_fields = Vec::new();

        struct Node {
            name: Option<Ident>,
            ty: Option<TypePath>,
            namespace: Option<String>,
        }

        for field in fields.iter() {
            let field_ident = field.ident.as_ref().expect("Expected named field");
            let mut node: Node = Node {
                name: None,
                ty: None,
                namespace: None,
            };

            // Process the field’s attributes.
            for attr in field.attrs.iter() {
                let id = attr.path().get_ident().map(|i| i.to_string());
                if let Some(id) = id {
                    let field_str = field_ident.to_string();
                    match id.as_str() {
                        "attribute" => {
                            let field_str = match attr.parse_args::<LitStr>() {
                                Ok(s) => match s.value().as_str() {
                                    "KebabCase" | "kebab-kase" => field_str.to_kebab_case(),
                                    "LowerCamelCase" | "lowerCamelCase" => {
                                        field_str.to_lower_camel_case()
                                    }
                                    "ShoutyKebabCase" | "SHOUTY-KEBAB-CASE" => {
                                        field_str.to_shouty_kebab_case()
                                    }
                                    // "snek" - What chicanery, what shenanigans - and dare I say it - what tomfoolery!
                                    "ShoutySnakeCase" | "SHOUTY_SNAKE_CASE" | "ShoutySnekCase"
                                    | "SHOUTY_SNEK_CASE" => field_str.to_shouty_snake_case(),
                                    "SnakeCase" | "snake_case" | "SnekCase" | "snek_case" => {
                                        field_str.to_snake_case()
                                    }

                                    "TitleCase" | "Title Case" => {
                                        panic!("XML does not allow the 'Title Case' casing scheme.")
                                    }
                                    "TrainCase" | "Train-Case" => field_str.to_train_case(),
                                    "UpperCamelCase" | "PascalCase" => {
                                        field_str.to_upper_camel_case()
                                    }
                                    r => panic!("Unknown case '{r}'"),
                                },
                                Err(_) => field_str,
                            };
                            ret.attribute_fields.push(quote! {
                                .attribute(#field_str, format!("{}", self.#field_ident))
                            })
                        }
                        "node" => {
                            node.name = Some(field_ident.clone());
                            if let Type::Path(path) = field.ty.clone() {
                                node.ty = Some(path);
                            };
                        }
                        "namespace" => {
                            // Field-level namespace is parsed as a literal.
                            let ns: LitStr = attr
                                .parse_args()
                                .expect("Expected string literal in namespace attribute");
                            node.namespace = Some(ns.value());
                        }
                        _ => {}
                    }
                }
            }

            if node.name.is_some() {
                node_fields.push(node);
            }
        }

        for node in node_fields {
            let namespace_stream = match node.namespace {
                Some(ns) => quote! {
                    .namespace(#ns).expect("Failed to set node namespace")
                },
                None => quote! {},
            };

            let node_name = node.name.expect("Unnamed node");

            let stream = match node.ty {
                Some(ty) => {
                    if type_is_vec(&ty) {
                        quote! {
                            .data(
                                self.#node_name.iter()
                                    .map(|d| flexml::XMLData::from(d.to_xml()#namespace_stream))
                                    .collect::<Vec<flexml::XMLData>>().as_slice()
                            )
                        }
                    } else {
                        quote! {
                            .datum(self.#node_name.to_xml_data()#namespace_stream)
                        }
                    }
                }
                None => {
                    panic!("Could not determine type of field {node_name}");
                }
            };

            ret.node_fields.push(stream);
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
