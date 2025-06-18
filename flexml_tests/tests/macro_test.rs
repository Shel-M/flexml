use flexml::macros::ToXML;
use flexml::{IntoXML, XML};

#[derive(ToXML)]
#[name("root")] // To manually name
// #[case("lowerCamelCase")] // To just change the rendered case scheme
#[namespaces(("Namespace1", "https://namespace1.com/namespace"), ("Namespace2", "https://namespace2.com/namespace"))]
#[namespace("Namespace1")]
struct ComplexStructRoot {
    #[case("lowerCamelCase")]
    data1: Vec<Node>,
    #[namespace("Namespace1")]
    data2: Node,

    #[attribute]
    #[name("Attrib1")]
    attrib1: String,
    #[attribute]
    #[case("UpperCamelCase")]
    #[namespace("Namespace1")]
    attrib2: &'static str,

    #[unserialized]
    unserialized_member: String,
}

#[derive(ToXML)]
struct Node {
    data1: String,
    #[with(prepend_foo)]
    data2: Vec<Node>,
}

impl Node {
    fn prepend_foo(&self) -> XML {
        XML::new("Node")
            .text(&"foo ".to_string())
            .text(&self.data1)
            .data(
                self.data2
                    .iter()
                    .map(|d| d.to_xml())
                    .collect::<Vec<flexml::XML>>()
                    .as_slice(),
            )
    }
}

#[test]
fn test_complex_struct() {
    let test_structure = ComplexStructRoot {
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
        r#"<n:root Attrib1="Attribute_value" n:Attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><node>First node, first datapoint</node><n:Node>String mixed with <Node>foo Second node, sub-datapoint</Node></n:Node></n:root>"#,
        test_structure.to_xml().to_string()
    )
}

#[test]
fn test_unit_struct() {
    #[derive(ToXML)]
    struct Root {
        data: Node,
    }

    #[derive(ToXML)]
    struct Node;

    let test_struct = Root { data: Node {} };

    assert_eq!("<Root><Node/></Root>", test_struct.to_xml().to_string());
}

#[test]
fn test_unit_struct_unit_repr() {
    #[derive(ToXML)]
    struct Root {
        data: Node,
    }

    #[derive(ToXML)]
    #[unit_repr(true)]
    struct Node;

    let test_struct = Root { data: Node {} };

    assert_eq!(
        "<Root><Node>true</Node></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_unit_struct_namespace() {
    #[derive(ToXML)]
    struct Root {
        data: Node,
    }

    #[derive(ToXML)]
    #[namespace("Namespace1")]
    struct Node;

    let test_struct = Root { data: Node {} };

    assert_eq!(
        "<Root xmlns:n=\"https://namespace1.com/namespace\"><n:Node/></Root>",
        test_struct.to_xml().to_string()
    );
}

// Tagged Enum Tests
#[derive(ToXML)]
#[name("Root")]
struct TaggedEnumRoot(NestedEnum);

#[derive(ToXML)]
#[namespaces(("options_namespace", "https://options_namespace.com/namespace"))]
#[untagged]
enum NestedEnum {
    TaggedOptions(TaggedOptions),
    UntaggedOptions(UntaggedOptions),
}

#[derive(ToXML)]
#[unit_repr(true)]
enum TaggedOptions {
    OneNamespacedSub(
        #[namespace("options_namespace")] TaggedOptionsNodeA,
        TaggedOptionsNodeB,
    ),
    TaggedNode(TaggedOptionsNodeA),
    Primitive(u16),
    NamedNode {
        #[case("PascalCase")]
        tag: TaggedOptionsNodeA,
    },
    NamedPrimitive {
        tag: u16,
    },
    #[case_all("PascalCase")]
    TwoNamed {
        tag_a: TaggedOptionsNodeA,
        tag_b: u16,
    },
    #[namespace("options_namespace")]
    NamespacedNode(TaggedOptionsNodeA),
    BoolUnit,
}

#[derive(ToXML)]
#[name("NodeA")]
struct TaggedOptionsNodeA {
    data: String,
}

#[derive(ToXML)]
#[name("NodeB")]
struct TaggedOptionsNodeB {
    #[name("Data")]
    data: u64,
}

#[test]
fn test_enum_tagged() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::TaggedNode(
        TaggedOptionsNodeA {
            data: "String".to_string(),
        },
    )));
    assert_eq!(
        "<Root><TaggedNode><NodeA>String</NodeA></TaggedNode></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_enum_primitive() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::Primitive(16)));
    assert_eq!(
        "<Root><Primitive>16</Primitive></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_enum_named_node() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::NamedNode {
        tag: TaggedOptionsNodeA {
            data: "String".to_string(),
        },
    }));
    assert_eq!(
        "<Root><NamedNode><Tag><NodeA>String</NodeA></Tag></NamedNode></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_enum_named_primitive() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::NamedPrimitive {
        tag: 16,
    }));
    assert_eq!(
        "<Root><NamedPrimitive><tag>16</tag></NamedPrimitive></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_enum_two_named_fields() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::TwoNamed {
        tag_a: TaggedOptionsNodeA {
            data: "String".to_string(),
        },
        tag_b: 16,
    }));
    assert_eq!(
        "<Root><TwoNamed><TagA><NodeA>String</NodeA></TagA><TagB>16</TagB></TwoNamed></Root>",
        test_struct.to_xml().to_string()
    );
}

#[test]
fn test_enum_namespaced_subnode() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::OneNamespacedSub(
        TaggedOptionsNodeA {
            data: "String".to_string(),
        },
        TaggedOptionsNodeB { data: 64 },
    )));

    assert_eq!(
        r#"<Root xmlns:o="https://options_namespace.com/namespace"><OneNamespacedSub><o:NodeA>String</o:NodeA><NodeB><Data>64</Data></NodeB></OneNamespacedSub></Root>"#,
        test_struct.to_xml().to_string()
    )
}

#[test]
fn test_enum_namespaced() {
    let test_struct = TaggedEnumRoot(NestedEnum::TaggedOptions(TaggedOptions::NamespacedNode(
        TaggedOptionsNodeA {
            data: "String".to_string(),
        },
    )));

    println!("{:?}", flexml::XMLNamespaces::hashmap().unwrap());
    let test_xml = test_struct.to_xml();
    println!("{:?}\n{:#?}", test_xml.namespaces(), test_xml);

    assert_eq!(
        r#"<Root xmlns:o="https://options_namespace.com/namespace"><o:NamespacedNode><NodeA>String</NodeA></o:NamespacedNode></Root>"#,
        test_xml.to_string()
    )
}

#[test]
fn test_enum_unit() {
    let test_value = TaggedOptions::BoolUnit;
    assert_eq!("<BoolUnit>true</BoolUnit>", test_value.to_xml().to_string());
}

#[derive(ToXML)]
#[untagged]
enum UntaggedOptions {
    Primitive(u64),
    Node(TaggedOptionsNodeA),
}

#[test]
fn test_untagged_enum_primitive() {
    let test_struct = TaggedEnumRoot(NestedEnum::UntaggedOptions(UntaggedOptions::Primitive(64)));

    assert_eq!("<Root>64</Root>", test_struct.to_xml().to_string())
}

#[test]
fn test_untagged_enum_node() {
    let test_struct = TaggedEnumRoot(NestedEnum::UntaggedOptions(UntaggedOptions::Node(
        TaggedOptionsNodeA {
            data: "String".into(),
        },
    )));

    assert_eq!(
        "<Root><NodeA>String</NodeA></Root>",
        test_struct.to_xml().to_string()
    )
}

#[derive(ToXML)]
#[case("PascalCase")]
#[untagged]
enum NamedEnum {
    Primitive(u16),
    #[with(repeat)]
    WithValue(u8),
}

impl NamedEnum {
    fn repeat(&self) -> XML {
        if let Self::WithValue(v) = self {
            let v = (0..*v).map(|i| i.to_xml()).collect::<Vec<XML>>();
            XML::new_untagged().data(v.as_slice())
        } else {
            self.to_xml()
        }
    }
}

#[test]
fn test_named_tagged_enum_primitive() {
    let test_value = NamedEnum::Primitive(16);

    assert_eq!("<NamedEnum>16</NamedEnum>", test_value.to_xml().to_string())
}

#[test]
fn test_named_tagged_enum_with() {
    let test_value = NamedEnum::WithValue(2);

    assert_eq!("<NamedEnum>01</NamedEnum>", test_value.to_xml().to_string())
}
