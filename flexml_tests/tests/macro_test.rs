use flexml::macros::XMLNode;
use flexml::{IntoXMLNode, XMLData, XMLNode};

#[test]
fn test_build_simple_xml() {
    #[derive(XMLNode)]
    #[name("root")]
    #[namespaces(("Namespace1", "https://namespace1.com/namespace"), ("Namespace2", "https://namespace2.com/namespace"))]
    #[namespace("Namespace1")]
    struct Root {
        #[node]
        data1: Vec<Node>,
        #[node]
        #[namespace("Namespace1")]
        data2: Node,

        #[attribute]
        attrib1: String,
        #[attribute("UpperCamelCase")]
        attrib2: &'static str,
    }

    #[derive(XMLNode)]
    struct Node {
        #[node]
        data1: String,
        #[node(with = foo)]
        data2: Vec<Node>,
    }

    impl Node {
        fn foo(&self) -> XMLData {
            XMLNode::new("Node")
                .text(&"foo ".to_string())
                .text(&self.data1)
                .into()
        }
    }

    let test_structure = Root {
        data1: vec![Node {
            data1: "First node, first datapoint".to_string(),
            data2: vec![],
        }],
        data2: Node {
            data1: String::from("String mixed with "),
            data2: vec![Node {
                data1: "Second node, sub-datapoint".to_string(),
                data2: vec![],
            }],
        },
        attrib1: "Attribute_value".to_string(),
        attrib2: "Attribute_value_2",
    };

    assert_eq!(
        r#"<n:root attrib1="Attribute_value" Attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node>String mixed with <Node>Second node, sub-datapoint</Node></n:Node></n:root>"#,
        test_structure.to_xml().to_string()
    )
}

#[test]
fn test_build_unit_struct() {
    #[derive(XMLNode)]
    struct Root {
        #[node]
        data: Node,
    }

    #[derive(XMLNode)]
    struct Node;

    let test_struct = Root { data: Node {} };

    assert_eq!("<Root><Node/></Root>", test_struct.to_xml().to_string());
}
