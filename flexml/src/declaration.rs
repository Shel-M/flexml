use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XMLDeclaration {
    Xml {
        version: (u32, u32),
        encoding: XMLEncoding,
    },
}

impl XMLDeclaration {
    #[must_use]
    pub const fn new(version: (u32, u32), encoding: XMLEncoding) -> Self {
        Self::Xml { version, encoding }
    }

    #[must_use]
    pub const fn version(mut self, version: (u32, u32)) -> Self {
        self.set_version(version);
        self
    }

    pub const fn set_version(&mut self, version: (u32, u32)) {
        match self {
            Self::Xml {
                version: (major, minor),
                encoding: _,
            } => (*major, *minor) = version,
        }
    }

    #[must_use]
    pub const fn encoding(mut self, encoding: XMLEncoding) -> Self {
        self.set_encoding(encoding);
        self
    }

    pub const fn set_encoding(&mut self, encoding: XMLEncoding) {
        match self {
            Self::Xml {
                version: _,
                encoding: ref mut current_encoding,
            } => *current_encoding = encoding,
        }
    }
}

impl Default for XMLDeclaration {
    fn default() -> Self {
        Self::new((1, 0), XMLEncoding::NotSpecified)
    }
}

impl Display for XMLDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xml { version, encoding } => {
                write!(f, r#"<?xml version="{}.{}" "#, version.0, version.1)?;
                encoding.fmt(f)?;
                write!(f, "?>")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum XMLEncoding {
    NotSpecified,
    UTF8,
}

impl Display for XMLEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSpecified => Ok(()),
            Self::UTF8 => write!(f, r#"encoding="UTF-8" "#),
        }
    }
}
