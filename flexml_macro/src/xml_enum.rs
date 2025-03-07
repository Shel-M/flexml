use syn::{DataEnum, Ident, LitStr};

#[derive(Debug, Default)]
pub(crate) struct EnumVariant {
    name: Option<Ident>,
    case: Option<String>,
    // ty: Option<TypePath>,
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
    pub(crate) fn process_fields(data_enum: &DataEnum, with: &Option<Ident>) -> Vec<Self> {
        let mut variants = Vec::new();

        for variant in &data_enum.variants {
            let mut enum_var = EnumVariant {
                name: Some(variant.ident.clone()),
                case: None,
                // ty: None,
                namespace: None,
                tagging: EnumTagging::Default,
                with: with.clone(),
            };

            for attr in variant.attrs.iter() {
                let id = attr.path().get_ident();
                if let Some(id) = id {
                    match id.to_string().as_str() {
                        "case" => {
                            match attr.parse_args::<LitStr>() {
                                Ok(s) => {
                                    // handling for nodes
                                    enum_var.case = Some(s.value());
                                }
                                Err(_) => panic!(
                                    "Could not parse attribute argument, expected string literal"
                                ),
                            };
                        }
                        "untagged" => enum_var.tagging = EnumTagging::Untagged,
                        "with" => {
                            let with: Ident = attr
                                .parse_args()
                                .expect("Expected identifier in with attribute");
                            enum_var.with = Some(with)
                        }
                        _ => {}
                    }
                }
            }

            variants.push(enum_var);
        }

        variants
    }
}
