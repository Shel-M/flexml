#![doc = include_str!("../README.md")]

pub mod data;
pub mod namespace;
pub mod node;

pub use data::*;
pub use namespace::*;
pub use node::*;

#[cfg(any(feature = "macro", test))]
pub use flexml_macro as macros;

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase as _, ToSnakeCase,
    ToTrainCase, ToUpperCamelCase,
};

use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

pub trait IntoXMLNode {
    fn to_xml(&self) -> XMLNode;
}

#[derive(Debug)]
pub enum XMLError {
    NamespaceNotFound(String),
    NamespaceOnText,
    Other(String),
}

impl Display for XMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        type S = XMLError;

        match self {
            S::NamespaceNotFound(v) => write!(
                f,
                "XMLError::NamespaceNotFound - Namespace \"{v}\" not defined"
            ),
            S::Other(v) => write!(f, "XMLError::Other \"{v}\""),
            XMLError::NamespaceOnText => write!(
                f,
                "XMLError::NamespaceOnText \"Cannot set namespace on text node.\""
            ),
        }
    }
}

impl<T: Error + Display> From<T> for XMLError {
    fn from(value: T) -> Self {
        Self::Other(value.to_string())
    }
}

pub(crate) fn conv_case<T: Display, U: Display>(input: T, case: U) -> String {
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
