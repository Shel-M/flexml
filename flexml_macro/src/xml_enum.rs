use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

use crate::{conv_case, DeriveAttributes};

#[derive(Debug)]
pub(crate) struct EnumVariant {
    alias: String,
    case: Option<String>,
    name: String,
    namespace: Option<String>,
    tagging: EnumTagging,
    with: Option<Ident>,
}

#[derive(Debug, Default)]
enum EnumTagging {
    #[default]
    Default,
    Untagged,
}

impl EnumVariant {
    pub(crate) fn process_fields(data_enum: &DataEnum, with: &Option<Ident>) -> Vec<TokenStream> {
        let mut variants = Vec::new();

        for xml_variant in &data_enum.variants {
            let mut variant = EnumVariant::from(DeriveAttributes::from(&xml_variant.attrs));
            variant.name = xml_variant.ident.to_string();
            if variant.with.is_none() {
                variant.with = with.clone()
            }

            for field in &xml_variant.fields {
                DeriveAttributes::from(&field.attrs);
            }

            variants.push(variant);
        }

        todo!()
    }
}

impl From<DeriveAttributes> for EnumVariant {
    fn from(value: DeriveAttributes) -> Self {
        Self {
            alias: value.name.unwrap_or(String::new()),
            case: value.case,
            name: String::new(),
            namespace: value.namespace,
            tagging: match value.untagged {
                true => EnumTagging::Untagged,
                false => EnumTagging::Default,
            },
            with: value.with,
        }
    }
}
