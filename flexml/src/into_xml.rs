use crate::XML;

pub trait IntoXML<'a> {
    fn to_xml(&self) -> XML<'a>;
}

impl<'a, T: IntoXML<'a>> IntoXML<'a> for Option<T> {
    fn to_xml(&self) -> XML<'a> {
        match self {
            Some(v) => v.to_xml(),
            None => XML::None,
        }
    }
}

// impl<'a, T: IntoXML<'a>> IntoXML<'a> for Vec<T> {
//     fn to_xml(&self) -> XML<'a> {
//         XML::new_untagged().data(self.clone())
//     }
// }

impl<'a> IntoXML<'a> for bool {
    fn to_xml(&self) -> XML<'a> {
        match self {
            true => "true".to_xml(),
            false => "false".to_xml(),
        }
    }
}

impl<'a> IntoXML<'a> for &str {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for String {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for u8 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for u16 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for u32 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for u64 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for u128 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for i8 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for i16 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for i32 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for i64 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}

impl<'a> IntoXML<'a> for i128 {
    fn to_xml(&self) -> XML<'a> {
        XML::Text(self.to_string())
    }
}
