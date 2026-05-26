use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse::Parse, punctuated::Punctuated, Attribute, Ident, Lit, LitStr, Token};

use crate::NamespaceTuple;

#[derive(Debug, Default)]
pub struct DeriveAttributes {
    pub attribute: bool,
    pub case: Option<String>,
    pub case_all: Option<String>,
    pub alias: Option<String>,
    pub namespace: Option<String>,
    pub namespaces: Vec<NamespaceTuple>,
    pub declaration: Option<DeclarationFormats>,
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
                    "declaration" => {
                        ret.declaration = Some(
                            attr.parse_args::<DeclarationFormats>()
                                .unwrap_or(DeclarationFormats::Empty),
                        );
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

#[derive(Debug)]
pub enum SupportedEncodingFormats {
    UTF8,
}

impl SupportedEncodingFormats {
    fn into_tokens(self) -> TokenStream {
        match self {
            Self::UTF8 => quote! { flexml::XMLEncoding::UTF8 },
        }
    }
}

#[derive(Debug)]
pub enum DeclarationFormats {
    Empty,
    Xml(u32, u32, SupportedEncodingFormats),
    XmlVersion(u32, u32),
    XmlEncoding(SupportedEncodingFormats),
}

impl DeclarationFormats {
    pub fn into_tokens(self) -> TokenStream {
        let mut out = quote! { flexml::XMLDeclaration:: default() };

        match self {
            Self::Empty => {}

            Self::Xml(major, minor, encoding_formats) => {
                let encoding = encoding_formats.into_tokens();
                out = quote! { flexml::XMLDeclaration::new((#major, #minor), #encoding ) };
            }
            Self::XmlVersion(major, minor) => out.append_all(quote! { .version((#major, #minor)) }),
            Self::XmlEncoding(encoding_format) => {
                let encoding_format = encoding_format.into_tokens();
                out.append_all(quote! { .encoding(#encoding_format) });
            }
        }

        out
    }

    fn version_string(value: &str) -> Result<(u32, u32), String> {
        let mut major = None;
        let mut minor = None;
        for part in value.split('.') {
            let Ok(part) = part.parse() else {
                return Err(format!(
                    "Unparsable version literal - {part} not an integer"
                ));
            };
            if major.is_none() {
                major = Some(part);
                continue;
            }
            if minor.is_none() {
                minor = Some(part);
                continue;
            }
            return Err(
                "Unparsable version literal - only major and minor version expected".to_string(),
            );
        }

        Ok((major.unwrap_or_default(), minor.unwrap_or_default()))
    }

    fn encoding_string(value: &str) -> Result<SupportedEncodingFormats, &'static str> {
        match value.to_lowercase().as_str() {
            "utf8" | "utf-8" => Ok(SupportedEncodingFormats::UTF8),
            _ => Err("Unknown or unsupported encoding format - only UTF-8 is supported"),
        }
    }
}

impl Parse for DeclarationFormats {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

        match args.len() {
            0 => Ok(Self::Empty),
            1 => {
                let val = args[0].value();
                if val.contains('.') {
                    match Self::version_string(&val) {
                        Ok((major, minor)) => return Ok(Self::XmlVersion(major, minor)),
                        Err(e) => return Err(input.error(e)),
                    }
                }

                match Self::encoding_string(&val) {
                    Ok(enc) => Ok(Self::XmlEncoding(enc)),
                    Err(e) => Err(input.error(e)),
                }
            }
            2 => {
                let (major, minor) = match Self::version_string(&args[0].value()) {
                    Ok((major, minor)) => (major, minor),
                    Err(e) => return Err(input.error(e)),
                };

                match Self::encoding_string(&args[1].value()) {
                    Ok(enc) => Ok(Self::Xml(major, minor, enc)),
                    Err(e) => Err(input.error(e)),
                }
            }
            _ => Err(input.error("Expected at most two string literals")),
        }
    }
}
