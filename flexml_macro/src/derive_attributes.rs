use syn::{punctuated::Punctuated, Attribute, Ident, LitStr, Token};

use crate::NamespaceTuple;

#[derive(Debug, Default)]
pub(crate) struct DeriveAttributes {
    pub attribute: bool,
    pub case: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub namespaces: Vec<NamespaceTuple>,
    pub with: Option<Ident>,
    pub unserialized: bool,
    pub untagged: bool,
}

impl From<&Vec<Attribute>> for DeriveAttributes {
    fn from(attrs: &Vec<Attribute>) -> Self {
        let mut ret = DeriveAttributes::default();

        for attr in attrs.iter() {
            let id = attr.path().get_ident().map(|i| i.to_string());
            if let Some(id) = id {
                match id.as_str() {
                    "attribute" => ret.attribute = true,
                    "case" => {
                        match attr.parse_args::<LitStr>() {
                            Ok(s) => {
                                ret.case = Some(s.value());
                            }
                            Err(_) => {
                                panic!("Could not parse #[case] argument, expected string literal")
                            }
                        };
                    }
                    "name" => {
                        ret.name = Some(
                            attr.parse_args::<LitStr>()
                                .expect("Expected string literal in namespace attribute")
                                .value(),
                        )
                    }
                    "namespace" => {
                        ret.namespace = Some(
                            attr.parse_args::<LitStr>()
                                .expect("Expected string literal in namespace attribute")
                                .value(),
                        )
                    }
                    "namespaces" => {
                        let namespaces: Punctuated<NamespaceTuple, Token![,]> = attr
                            .parse_args_with(Punctuated::parse_terminated)
                            .expect("Failed to parse namespaces attribute");

                        ret.namespaces.extend(namespaces);
                    }
                    "with" => {
                        ret.with = Some(
                            attr.parse_args::<Ident>()
                                .expect("Expected identifier in with attribute"),
                        )
                    }
                    "unserialized" => {
                        ret.unserialized = true;
                        break; // Don't really need to continue processing in this case.
                    }
                    "untagged" => ret.untagged = true,
                    _ => {}
                }
            }
        }

        ret
    }
}
