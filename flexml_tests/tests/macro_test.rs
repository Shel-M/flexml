use flexml::macros::XMLNode;
use flexml::{IntoXMLNode, ToXMLData, XMLData, XMLNode};

#[test]
fn test_struct_xml() {
    #[derive(XMLNode)]
    // #[name("root")] // To manually name
    #[case("lowerCamelCase")] // To just change the rendered case scheme
    #[namespaces(("Namespace1", "https://namespace1.com/namespace"), ("Namespace2", "https://namespace2.com/namespace"))]
    #[namespace("Namespace1")]
    struct Root {
        #[case("lowerCamelCase")]
        data1: Vec<Node>,
        #[namespace("Namespace1")]
        data2: Node,

        #[attribute]
        attrib1: String,
        #[attribute]
        #[case("UpperCamelCase")]
        attrib2: &'static str,

        #[unserialized]
        unserialized_member: String,
    }

    #[derive(XMLNode)]
    struct Node {
        data1: String,
        #[with(prepend_foo)]
        data2: Vec<Node>,
    }

    impl Node {
        fn prepend_foo(&self) -> XMLData {
            XMLNode::new("Node")
                .text(&"foo ".to_string())
                .text(&self.data1)
                .data(
                    self.data2
                        .iter()
                        .map(|d| d.to_xml_data())
                        .collect::<Vec<flexml::XMLData>>()
                        .as_slice(),
                )
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
        unserialized_member: "Unserialized".to_string(),
    };

    print!("{}", test_structure.unserialized_member);

    assert_eq!(
        r#"<n:root attrib1="Attribute_value" Attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><node>First node, first datapoint</node><n:Node>String mixed with <Node>foo Second node, sub-datapoint</Node></n:Node></n:root>"#,
        test_structure.to_xml().to_string()
    )
}

#[test]
fn test_unit_struct() {
    #[derive(XMLNode)]
    struct Root {
        data: Node,
    }

    #[derive(XMLNode)]
    struct Node;

    let test_struct = Root { data: Node {} };

    assert_eq!("<Root><Node/></Root>", test_struct.to_xml().to_string());
}

// Enum Tests
#[derive(XMLNode)]
struct Root {
    data: NodeOptions,
}

#[derive(XMLNode)]
#[namespaces(("options_namespace", "https://options_namespace.com/namespace"))]
enum NodeOptions {
    OneNamespacedSub(#[namespace("options_namespace")] NodeA, NodeB),
    TaggedNode(NodeA),
    Primitive(u16),
    // NamedNode {
    //     #[case("PascalCase")]
    //     tag: NodeA,http
    // },
    // NamedPrimitive {
    //     tag: u16,
    // },
    // NamespacedNode(NodeA),
}

// impl IntoXMLNode for NodeOptions {
//     fn to_xml(&self) -> XMLNode {
//         match self {
//             Self::TaggedNode(n) => XMLNode::new("TaggedNode").datum(n.to_xml_data()),
//             // Self::UntaggedNode(n) => n.to_xml(),
//             Self::Primitive(n) => XMLNode::new("Primitive").datum(n.to_xml_data()),
//             // Self::NamedNode { tag } => {
//             //     XMLNode::new("NamedNode").datum(XMLNode::new("Tag").datum(tag.to_xml_data()).into())
//             // }
//             // Self::NamedPrimitive { tag } => XMLNode::new("NamedPrimitive")
//             //     .datum(XMLNode::new("tag").datum(tag.to_xml_data()).into()),
//             Self::OneNamespacedSub(n0, n1) => XMLNode::new("OneNamespacedSub")
//                 .datum(n0.to_xml_data().namespace("namespace").expect(""))
//                 .datum(n1.to_xml_data()),
//         }
//     }
// }

#[derive(XMLNode)]
struct NodeA {
    data: String,
}

#[derive(XMLNode)]
struct NodeB {
    data: u64,
}

#[test]
fn test_tagged_enum() {
    let test_struct = Root {
        data: NodeOptions::TaggedNode(NodeA {
            data: "String".to_string(),
        }),
    };
    assert_eq!(
        "<Root><TaggedNode><NodeA>String</NodeA></TaggedNode></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_primitive_enum() {
    let test_struct = Root {
        data: NodeOptions::Primitive(16),
    };
    assert_eq!(
        "<Root><Primitive>16</Primitive></Root>",
        test_struct.to_xml().to_string()
    );
}

// #[test]
// fn test_named_node_enum() {
//     let test_struct = Root {
//         data: NodeOptions::NamedNode {
//             tag: NodeA {
//                 data: "String".to_string(),
//             },
//         },
//     };
//     assert_eq!(
//         "<Root><NamedNode><Tag><NodeA>String</NodeA></Tag></NamedNode></Root>",
//         test_struct.to_xml().to_string()
//     );
// }
//
// #[test]
// fn test_named_primitive_enum() {
//     let test_struct = Root {
//         data: NodeOptions::NamedPrimitive { tag: 16 },
//     };
//     assert_eq!(
//         "<Root><NamedPrimitive><tag>16</tag></NamedPrimitive></Root>",
//         test_struct.to_xml().to_string()
//     );
// }

#[test]
fn test_namespaced_subnode_enum() {
    let test_struct = Root {
        data: NodeOptions::OneNamespacedSub(
            NodeA {
                data: "String".to_string(),
            },
            NodeB { data: 64 },
        ),
    };

    assert_eq!(
        r#"<Root xmlns:o="https://options_namespace.com/namespace"><OneNamespacedSub><o:NodeA>String</o:NodeA><NodeB>64</NodeB></OneNamespacedSub></Root>"#,
        test_struct.to_xml().to_string()
    )
}

// Todo: Untagged enum tests
