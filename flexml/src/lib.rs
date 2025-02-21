#![doc = include_str!("../README.md")]

pub mod data;
pub mod namespace;
pub mod node;

pub use data::*;
pub use namespace::*;
pub use node::*;

#[cfg(any(feature = "macro", test))]
pub use flexml_macro as macros;

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
