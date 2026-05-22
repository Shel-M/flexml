use syn::{punctuated::Punctuated, Attribute, Ident, Lit, LitStr, Token};

use crate::NamespaceTuple;

#[derive(Debug, Default)]
pub struct DeriveAttributes {
    pub attribute: bool,
    pub case: Option<String>,
    pub case_all: Option<String>,
    pub alias: Option<String>,
    pub namespace: Option<String>,
    pub namespaces: Vec<NamespaceTuple>,
    pub with: Option<Ident>,
    pub unit_repr: Option<Lit>,
    pub unserialized: bool,
    pub untagged: bool,
}

#[allow(clippy::fallible_impl_from)] // Panics in macros show as editor errors
impl From<&Vec<Attribute>> for DeriveAttributes {
    fn from(attrs: &Vec<Attribute>) -> Self {
        let mut ret = Self::default();

        for attr in attrs {
            let id = attr
                .path()
                .get_ident()
                .map(std::string::ToString::to_string);
            if let Some(id) = id {
                match id.as_str() {
                    "attribute" => ret.attribute = true,
                    "case" => {
                        match attr.parse_args::<LitStr>() {
                            Ok(s) => {
                                ret.case = Some(s.value());
                            }
                            Err(e) => {
                                panic!("Could not parse #[case] argument, expected string literal - {e}")
                            }
                        }
                    }
                    "case_all" => match attr.parse_args::<LitStr>() {
                        Ok(s) => {
                            ret.case_all = Some(s.value());
                        }
                        Err(e) => {
                            panic!(
                                    "Could not parse #[case_all] argument, expected string literal - {e}"
                                )
                        }
                    },
                    "name" => {
                        ret.alias = Some(
                            attr.parse_args::<LitStr>()
                                .expect("Expected string literal in namespace attribute")
                                .value(),
                        );
                    }
                    "namespace" => {
                        ret.namespace = Some(
                            attr.parse_args::<LitStr>()
                                .expect("Expected string literal in namespace attribute")
                                .value(),
                        );
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
                        );
                    }
                    "unit_repr" => {
                        ret.unit_repr =
                            Some(attr.parse_args::<Lit>().expect("Expected literal value"));
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
