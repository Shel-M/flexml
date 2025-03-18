use crate::XML;

pub trait IntoXML {
    fn to_xml(&self) -> XML;
}

impl<T: IntoXML> IntoXML for Option<T> {
    fn to_xml(&self) -> XML {
        match self {
            Some(v) => v.to_xml(),
            None => XML::None,
        }
    }
}

impl IntoXML for bool {
    fn to_xml(&self) -> XML {
        match self {
            true => "true".to_xml(),
            false => "false".to_xml(),
        }
    }
}

impl IntoXML for &str {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for String {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for u8 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for u16 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for u32 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for u64 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for u128 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for i8 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for i16 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for i32 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for i64 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}

impl IntoXML for i128 {
    fn to_xml(&self) -> XML {
        XML::Text(self.to_string())
    }
}
