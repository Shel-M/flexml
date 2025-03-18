use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, Ident};

use crate::{conv_case, DeriveAttributes, XMLAttributes};

#[derive(Debug)]
pub(crate) struct EnumHandler {
    pub variant_tokens: Vec<TokenStream>,
}

impl EnumHandler {
    pub(crate) fn expand_tokens(
        data_enum: &DataEnum,
        xml_attributes: &XMLAttributes,
    ) -> TokenStream {
        let xml_enum_variants = EnumHandler::process_fields(data_enum, xml_attributes);

        let node_tag = xml_attributes.get_node_tag();
        if data_enum.variants.is_empty() {
            let unit_repr = &xml_attributes.unit_repr;
            quote! {
                flexml::XML::new(#node_tag) .datum(#unit_repr) ,
            };
        }
        let variant_tokens = xml_enum_variants.variant_tokens;
        if xml_attributes.alias.is_some() || xml_attributes.case.is_some() {
            let node_ns_token = &xml_attributes.namespace_token;

            quote! {
                flexml::XML::new(#node_tag)
                    #node_ns_token
                    .datum(
                        match self {
                            #(#variant_tokens)*
                        })
            }
        } else {
            quote! {
                match self {
                    #(#variant_tokens)*
                }
            }
        }
    }

    pub(crate) fn process_fields(data_enum: &DataEnum, xml_attributes: &XMLAttributes) -> Self {
        let mut variant_tokens = Vec::new();

        for xml_variant in &data_enum.variants {
            let mut variant = EnumVariant::from(DeriveAttributes::from(&xml_variant.attrs));
            variant.untagged = xml_attributes.untagged;
            variant.name = Some(xml_variant.ident.clone());
            if variant.case.is_none() {
                variant.case = xml_attributes.case_all.clone();
            }

            if variant.alias.is_empty() {
                variant.alias = xml_variant.ident.clone().to_string();
                if variant.case.is_some() {
                    variant.alias = conv_case(variant.alias, variant.case.as_ref().unwrap())
                }
            }

            let field_tokens = match &xml_variant.fields {
                syn::Fields::Named(fields_named) => {
                    variant.named_fields_to_tokens(fields_named, &variant.case_all)
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    variant.unnamed_fields_to_tokens(fields_unnamed, &variant.case_all)
                }
                syn::Fields::Unit => variant.unit_fields_to_tokens(xml_attributes),
            };

            variant_tokens.push(field_tokens)
        }

        EnumHandler { variant_tokens }
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    alias: String,
    case: Option<String>,
    case_all: Option<String>,
    name: Option<Ident>,
    namespace: Option<String>,
    untagged: bool,
    with: Option<Ident>,
}

impl EnumVariant {
    fn named_fields_to_tokens(
        &self,
        fields: &FieldsNamed,
        case_all: &Option<String>,
    ) -> TokenStream {
        let fields = std::convert::Into::<Fields>::into(fields.clone());

        let variant_name = &self.name;
        let conv_call = match &self.with {
            Some(ref with) => quote! {.#with()},
            None => quote! {.to_xml()},
        };

        let mut field_names = Vec::new();
        let mut field_tokens = Vec::new();
        for field in fields {
            let field_name = field.ident.as_ref().unwrap();
            field_names.push(field_name.clone());

            let mut field_attributes = DeriveAttributes::from(&field.attrs);
            if field_attributes.case.is_none() {
                field_attributes.case = case_all.clone();
            }

            let alias = if field_attributes.alias.is_some() {
                field_attributes.alias.unwrap()
            } else if field_attributes.case.is_some() {
                conv_case(field_name, field_attributes.case.as_ref().unwrap())
            } else {
                format! {"{field_name}"}
            };

            let namespace_stream = field_attributes.namespace.map(|ns| {
                quote! {
                    .namespace(#ns).expect("Failed to set node namespace.")
                }
            });
            field_tokens.push(if self.untagged {
                quote! { flexml::XML::new_container().datum(#field_name #conv_call)#namespace_stream }
            } else {
                quote! { flexml::XML::new(#alias).datum(#field_name #conv_call)#namespace_stream}
            });
        }
        if self.untagged {
            quote! {Self::#variant_name{#(#field_names,)*} => #((#field_tokens))* ,}
        } else {
            let variant_alias = &self.alias;

            let namespace_stream = self.namespace.as_ref().map(|ns| {
                quote! {
                    .namespace(#ns).expect("Failed to set node namespace.")
                }
            });
            quote! {
                Self::#variant_name{#(#field_names,)*} =>
                flexml::XML::new(#variant_alias) #namespace_stream #(.datum(#field_tokens))* ,
            }
        }
    }

    fn unnamed_fields_to_tokens(
        &self,
        fields: &FieldsUnnamed,
        case_all: &Option<String>,
    ) -> TokenStream {
        let fields = std::convert::Into::<Fields>::into(fields.clone());

        let variant_name = &self.name;

        let matching = (0..fields.len())
            .map(|i| {
                let n = format_ident!("n{i}");
                quote! {#n}
            })
            .collect::<Vec<TokenStream>>();

        let mut field_tokens = Vec::new();
        for (i, field) in fields.iter().enumerate() {
            let mut field_attributes = DeriveAttributes::from(&field.attrs);
            if field_attributes.case.is_none() {
                field_attributes.case = case_all.clone();
            }
            let n = format_ident!("n{i}");

            let conv_call = match &field_attributes.with {
                Some(ref with) => quote! {.#with()},
                None => quote! {.to_xml()},
            };
            let namespace_stream = field_attributes.namespace.map(|ns| {
                quote! {
                    .namespace(#ns).expect("Failed to set node namespace.")
                }
            });
            field_tokens.push(quote! {#n #conv_call #namespace_stream})
        }

        match &self.with {
            Some(ref with) => quote! {Self::#variant_name(#(#matching,)*) => self.#with(),},
            None => {
                if self.untagged {
                    quote! {Self::#variant_name(#(#matching,)*) => flexml::XML::new_untagged()#(.datum(#field_tokens))* ,}
                } else {
                    let variant_alias = &self.alias;

                    let namespace_stream = self.namespace.as_ref().map(|ns| {
                        quote! {
                            .namespace(#ns).expect("Failed to set node namespace.")
                        }
                    });
                    quote! {Self::#variant_name(#(#matching,)*) => flexml::XML::new(#variant_alias) #namespace_stream #(.datum(#field_tokens))*,}
                }
            }
        }
    }

    fn unit_fields_to_tokens(&self, xml_attributes: &XMLAttributes) -> TokenStream {
        let variant_name = &self.name;
        let conv_call = match &self.with {
            Some(ref with) => quote! {.#with()},
            None => quote! {.to_xml()},
        };

        let unit_repr = &xml_attributes.unit_repr;
        if self.untagged {
            quote! {Self::#variant_name => flexml::XML::new_untagged().datum(#unit_repr #conv_call) ,}
        } else {
            let variant_alias = &self.alias;

            let namespace_stream = self.namespace.as_ref().map(|ns| {
                quote! {
                    .namespace(#ns).expect("Failed to set node namespace.")
                }
            });
            quote! {
                Self::#variant_name =>
                flexml::XML::new(#variant_alias) #namespace_stream .datum(#unit_repr #conv_call) ,
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
            alias: value.alias.unwrap_or_default(),
            case: value.case,
            case_all: value.case_all,
            name: None,
            namespace: value.namespace,
            untagged: false,
            with: value.with,
        }
    }
}
