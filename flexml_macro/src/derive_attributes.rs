use syn::{punctuated::Punctuated, Attribute, Ident, LitStr, Token};

use crate::NamespaceTuple;

#[derive(Debug)]
pub(crate) enum DeriveAttributes {
    Attribute,
    Case(String),
    Name(String),
    Namespace(String),
    Namespaces(Vec<NamespaceTuple>),
    With(Ident),
    Unserialized,
    Untagged,
}

impl DeriveAttributes {
    pub(crate) fn from_vec(attrs: &[Attribute]) -> Vec<Self> {
        let mut ret = Vec::new();

        for attr in attrs.iter() {
            let id = attr.path().get_ident().map(|i| i.to_string());
            if let Some(id) = id {
                match id.as_str() {
                    "attribute" => {
                        ret.push(Self::Attribute);
                    }
                    "case" => {
                        match attr.parse_args::<LitStr>() {
                            Ok(s) => {
                                ret.push(Self::Case(s.value()));
                            }
                            Err(_) => {
                                panic!("Could not parse #[case] argument, expected string literal")
                            }
                        };
                    }
                    "name" => {
                        let n: LitStr = attr
                            .parse_args()
                            .expect("Expected string literal in namespace attribute");
                        ret.push(Self::Name(n.value()));
                    }
                    "namespace" => {
                        // For the struct-level namespace, parse as a string literal.
                        let ns: LitStr = attr
                            .parse_args()
                            .expect("Expected string literal in namespace attribute");
                        ret.push(Self::Namespace(ns.value()));
                    }
                    "namespaces" => {
                        let namespaces: Punctuated<NamespaceTuple, Token![,]> = attr
                            .parse_args_with(Punctuated::parse_terminated)
                            .expect("Failed to parse namespaces attribute");

                        let mut v = Vec::new();
                        for namespace in namespaces {
                            v.push(NamespaceTuple {
                                ns: namespace.ns,
                                uri: namespace.uri,
                            });
                        }

                        ret.push(Self::Namespaces(v));
                    }
                    "with" => {
                        let with: Ident = attr
                            .parse_args()
                            .expect("Expected identifier in with attribute");
                        ret.push(Self::With(with))
                    }
                    "unserialized" => {
                        ret.push(Self::Unserialized);
                        break; // Don't really need to continue processing in this case.
                    }
                    "untagged" => ret.push(Self::Untagged),
                    _ => {}
                }
            }
        }

        ret
    }
}
