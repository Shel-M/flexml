use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{DataEnum, Fields, FieldsUnnamed, Ident};

use crate::{conv_case, DeriveAttributes};

#[derive(Debug, Default)]
pub(crate) enum EnumTagging {
    #[default]
    Default,
    Untagged,
}

#[derive(Debug)]
pub(crate) struct EnumVariantTokens {
    pub variant_tokens: Vec<TokenStream>,
}

impl EnumVariantTokens {
    pub(crate) fn process_fields(data_enum: &DataEnum, untagged: bool) -> Self {
        let mut variant_tokens = Vec::new();

        for xml_variant in &data_enum.variants {
            let mut variant = EnumVariant::from(DeriveAttributes::from(&xml_variant.attrs));
            variant.untagged = untagged;
            variant.name = Some(xml_variant.ident.clone());
            if variant.alias.is_empty() {
                variant.alias = xml_variant.ident.clone().to_string();
                if variant.case.is_some() {
                    variant.alias = conv_case(variant.alias, variant.case.as_ref().unwrap())
                }
            }

            let mut field_tokens = match &xml_variant.fields {
                syn::Fields::Named(fields_named) => todo!(),
                syn::Fields::Unnamed(fields_unnamed) => {
                    variant.unnamed_fields_to_tokens(fields_unnamed)
                }
                syn::Fields::Unit => todo!(),
            };

            variant_tokens.push(field_tokens)
        }

        EnumVariantTokens { variant_tokens }
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    alias: String,
    case: Option<String>,
    name: Option<Ident>,
    namespace: Option<String>,
    untagged: bool,
    with: Option<Ident>,

    fields: Vec<TokenStream>,
}

impl EnumVariant {
    fn unnamed_fields_to_tokens(&self, fields: &FieldsUnnamed) -> TokenStream {
        let fields = std::convert::Into::<Fields>::into(fields.clone());

        let variant_name = &self.name;
        let conv_call = match (&self.with, self.untagged) {
            (Some(ref with), _) => quote! {.#with()},
            (None, true) => quote! {.to_xml()},
            (None, false) => quote! {.to_xml_data()},
        };

        let matching = (0..fields.len())
            .map(|i| {
                let n = format_ident!("n{i}");
                quote! {#n}
            })
            .collect::<Vec<TokenStream>>();

        let mut field_tokens = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            let field_attributes = DeriveAttributes::from(&field.attrs);
            let n = format_ident!("n{i}");

            let namespace_stream = field_attributes.namespace.map(|ns| {
                quote! {
                    .namespace(#ns).expect("Failed to set node namespace.")
                }
            });
            field_tokens.push(if self.untagged {
                todo!()
            } else {
                quote! {.datum( #n #conv_call #namespace_stream)}
            })
        }
        if self.untagged {
            quote! {Self::#variant_name(#(#matching,)*) => n0 #conv_call,}
        } else {
            let variant_alias = &self.alias;
            quote! {
                Self::#variant_name(#(#matching,)*) =>
                flexml::XMLNode::new(#variant_alias) #(#field_tokens)* ,
            }
        }
    }
}

impl From<DeriveAttributes> for EnumVariant {
    fn from(value: DeriveAttributes) -> Self {
        if value.untagged {
            panic!(
                "Incorrect placement of #[untagged] attribute, \
                it should be on the containing enum."
            )
        }
        Self {
            alias: value.name.unwrap_or_default(),
            case: value.case,
            name: None,
            namespace: value.namespace,
            untagged: false,
            with: value.with,

            fields: Vec::new(),
        }
    }
}

// #[derive(Debug)]
// pub(crate) struct EnumVariantField {
//     alias: String,
//     case: Option<String>,
//     name: Option<Ident>,
//     namespace: Option<String>,
//     tagging: EnumTagging,
//     with: Option<Ident>,
// }
//
// impl EnumVariantField {
//     fn to_tokens(&self, field_num: usize) -> TokenStream {
//         if self.name.is_some() {
//             let alias = if self.alias.is_empty() {
//                 match self.case {
//                     Some(ref case) => {
//                         conv_case(self.name.as_ref().expect("No name for variant field"), case)
//                     }
//                     None => self
//                         .name
//                         .as_ref()
//                         .expect("No name for variant field")
//                         .to_string(),
//                 }
//             } else {
//                 self.alias.to_string()
//             };
//         }
//         // Two Tuple Cases
//         // 1: XMLNode.datum(n.to_xml_data())
//         // 2: n.to_xml()
//         else {
//
//         }
//
//         quote! {}
//     }
// }
//
// impl From<DeriveAttributes> for EnumVariantField {
//     fn from(value: DeriveAttributes) -> Self {
//         Self {
//             alias: value.name.unwrap_or_default(),
//             case: value.case,
//             name: None,
//             namespace: value.namespace,
//             tagging: match value.untagged {
//                 true => EnumTagging::Untagged,
//                 false => EnumTagging::Default,
//             },
//             with: value.with,
//         }
//     }
// }
