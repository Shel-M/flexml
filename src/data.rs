use super::XMLNode;

use std::fmt::Display;

#[derive(Debug)]
pub enum XMLData {
    None,
    Nodes(Vec<XMLNode>),
    Text(String),
}

impl Display for XMLData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Nodes(nodes) => {
                for node in nodes {
                    node.fmt(f)?
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
