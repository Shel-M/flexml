An XML writer (maybe it'll have read someday, but for now I recommend [quick-xml] for deserializing) library that should be quick and easy to implement, with ergonomics and flexibility as the core goal.

# Why make this when [quick-xml] exists?
I personally don't like how quick-xml handles writing. It's very fast, stable, and well supported. It also isn't very easy to use to write, and its documentation for that use-case is generally lacking. It also [doesn't effectively support namespaces.](https://github.com/tafia/quick-xml/issues/218)
Quick-xml (especially with serde) is extremely good for reading XML.

# Why no [serde] feature?
Two reasons:
- There's no deserializer implemented for flexml
- Some of quick-xmls issues with supporting a few XML features are stated as not being particularly nice for the XML spec.

If you'd like to change this, you're welcome to submit a pull request, and I am welcome to deny it.

# Features
`macros`: Enables the flexml::macros::XMLNode procedural macro to implement the [IntoXMLNode] traits.

# Examples
Macro usage example
```rust
use flexml::IntoXMLNode;
use flexml::macros::XMLNode;

#[derive(XMLNode)]

// The default will match the struct name, this tag overrides.
#[name("foo")]

// This stores available namespaces. When serializing, only used namespaces will be rendered into the final document.
#[namespaces(("Namespace1", "https://namespace1.com/namespace"), ("Namespace2", "https://namespace2.com/namespace"))]

// This is how you tag a default namespace on a node.
#[namespace("Namespace1")]
struct Foo {
    // Multiple nodes can be defined. They'll be serialized in the order they appear on the struct.
    // A node tag on a Vec<Bar> (in this example) preserves the order of the Vec when serializing.
    #[node]
    data1: Vec<Node>,
    #[node]
    // A child-specific namespace - overrides a struct's default namespace.
    #[namespace("Namespace1")]
    data2: Node,

    // Attributes
    #[attribute]
    attrib1: String,
    // Display is used to convert attributes
    #[attribute]
    attrib2: &'static str,
}

#[derive(XMLNode)]
struct Node {
    // Nodes are inserted in-order, so you can accomplish mixed media

    // Note: A #[namespace] tag on a Text node like this one will panic at runtime.
    #[node]
    data1: String,
    #[node]
    data2: Vec<Node>,
}

fn foo() {
    let test_structure = Foo {
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
        r#"<n:foo attrib1="Attribute_value" attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node>String mixed with <Node>Second node, sub-datapoint</Node></n:Node></n:foo>"#,
        test_structure.to_xml().to_string()
    )
}
```

Which is equivalent to this non-macro implementation

```rust
use flexml::IntoXMLNode;

struct Root {
    data1: Vec<Node>,
    data2: Node,

    attrib1: String,
    attrib2: &'static str,
}

impl IntoXMLNode for Root {
    fn to_xml(&self) -> flexml::XMLNode {
        use flexml::ToXMLData;
        flexml::XMLNamespaces::insert("Namespace1", "https://namespace1.com/namespace")
            // This is why the macro can panic at runtime. The only time this should error is in the event of a RWLock poison error, which should be very rare.
            .expect("failed to insert namespace");
        flexml::XMLNamespaces::insert("Namespace2", "https://namespace2.com/namespace")
            .expect("failed to insert namespace");

        let node = flexml::XMLNode::new("root")
            .attribute("attrib1", &self.attrib1)
            .attribute("attrib2", &self.attrib2)
            .namespace("Namespace1")
            .expect("Failed to set doc namespace")
            .data(
                self.data1
                    .iter()
                    .map(|d| flexml::XMLData::from(d.to_xml()))
                    .collect::<Vec<flexml::XMLData>>()
                    .as_slice(),
            )
            .datum(
                self.data2
                    .to_xml_data()
                    .namespace("Namespace1")
                    .expect("Failed to set node namespace"),
            );

        node
    }
}

struct Node {
    data1: String,
    data2: Vec<Node>,
}

impl IntoXMLNode for Node {
    fn to_xml(&self) -> flexml::XMLNode {
        use flexml::ToXMLData;

        let node = flexml::XMLNode::new("Node")
            .datum(self.data1.to_xml_data())
            .data(
                self.data2
                    .iter()
                    .map(|d| flexml::XMLData::from(d.to_xml()))
                    .collect::<Vec<flexml::XMLData>>()
                    .as_slice(),
            );
        node
    }
}

fn foo() {
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
        r#"<n:root attrib1="Attribute_value" attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node>String mixed with <Node>Second node, sub-datapoint</Node></n:Node></n:root>"#,
        test_structure.to_xml().to_string()
    )
}
```

 [quick-xml]: https://docs.rs/quick-xml/latest/quick_xml/
 [serde]: https://serde.rs/

