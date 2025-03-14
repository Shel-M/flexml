An XML writer (maybe it'll have read someday, but for now I recommend [quick-xml] for deserializing) library that should be quick and easy to implement, with ergonomics and flexibility as the core goal.

# Why make this when [quick-xml] exists?
I personally don't like how quick-xml handles writing. It's very fast, stable, and well supported. It also isn't very easy to use to write, and its documentation for that use-case is generally lacking. It also [doesn't effectively support namespaces.](https://github.com/tafia/quick-xml/issues/218)
Quick-xml (especially with serde) is extremely good for reading XML.

# Why no [serde] feature?
Two reasons:
- There's no deserializer implemented for flexml
- Some of quick-xmls issues with supporting a few XML features are stated as not being particularly nice for the XML spec.

If you'd like to change this, you're welcome to submit a pull request, and I am welcome to deny it if I don't like it.

# Features
`macros`: Enables the flexml::macros::XMLNode procedural macro to implement the [IntoXMLNode] trait.

# Examples
Macro usage example
```rust
use flexml::macros::ToXML;
use flexml::{IntoXML, XML};

#[derive(ToXML)]

// The default will match the struct name, this tag overrides.
#[name("foo")]

// This stores available namespaces. When serializing, only used 
// namespaces will be rendered into the final document.
#[namespaces(("Namespace1", "https://namespace1.com/namespace"),
    ("Namespace2", "https://namespace2.com/namespace"))]

// This is how you tag a default namespace on a node.
#[namespace("Namespace1")]
struct Foo {
    // Multiple nodes can be defined. They'll be serialized in the
    // order they appear on the struct.
    // A node tag on a Vec<Bar> (in this example) preserves the 
    // order of the Vec when serializing.
    data1: Vec<Node>,
    // A child-specific namespace - overrides a struct's default
    // namespace.
    #[namespace("Namespace1")]
    data2: Node,

    // Display is used to convert attributes
    #[attribute]
    #[name("Attrib1")] // #[name] can be used to manually alias a field
    attrib1: String,
    // A case string may be passed into attributes. 
    // See [heck] for supported casing schemes.
    #[attribute("UpperCamelCase")]
    attrib2: &'static str,

    #[unserialized]
    unserialized_field: String,
}

#[derive(ToXML)]
struct Node {
    // Nodes are inserted in-order, so you can use mixed media.

    // Note: A #[namespace] tag on a Text node like this one will 
    // panic at runtime.
    data1: String,
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
        unserialized_field: "Unserialized".to_string(),
    };

    assert_eq!(
        r#"<n:foo Attrib1="Attribute_value" Attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node>String mixed with <Node>Second node, sub-datapoint</Node></n:Node></n:foo>"#,
        test_structure.to_xml().to_string()
    )
}
```

Which is equivalent to this non-macro implementation

```rust
use flexml::XML;
use flexml::IntoXML;
use flexml::XMLNamespaces;

struct Root {
    data1: Vec<Node>,
    data2: Node,

    attrib1: String,
    attrib2: &'static str,
}

impl IntoXML for Root {
    fn to_xml(&self) -> XML {
        XMLNamespaces::insert("Namespace1",
            "https://namespace1.com/namespace")
            // This is why the macro can panic at runtime. 
            // The only time this should error is in the event of a 
            // RWLock poison error, which should be very rare.
            .expect("failed to insert namespace");
        XMLNamespaces::insert("Namespace2",
            "https://namespace2.com/namespace")
            .expect("failed to insert namespace");

        let data1_nodes: Vec<XML> = self.data1.iter()
            .map(|n| n.to_xml()).collect();

        XML::new("root")
            .attribute("attrib1", &self.attrib1)
            .attribute("Attrib2", &self.attrib2)
            .namespace("Namespace1").expect("Failed to set doc namespace")
            .nodes(&data1_nodes)
            .node(
                self.data2
                    .to_xml()
                    .namespace("Namespace1")
                    .expect("Failed to set node namespace"),
            )
    }
}

struct Node {
    data1: String,
    data2: Vec<Node>,
}

impl IntoXML for Node {
    fn to_xml(&self) -> XML {
        XML::new("Node")
            .text(&self.data1)
            // You can also use .data() or .datum().
            // Convert the type with .to_xml().
            .data(
                self.data2
                    .iter()
                    .map(|d| d.to_xml())
                    .collect::<Vec<XML>>()
                    .as_slice(),
            )
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
        r#"<n:root attrib1="Attribute_value" Attrib2="Attribute_value_2" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node>String mixed with <Node>Second node, sub-datapoint</Node></n:Node></n:root>"#,
        test_structure.to_xml().to_string()
    )
}
```

[quick-xml]: https://docs.rs/quick-xml/latest/quick_xml/
[serde]: https://serde.rs/
[heck]: https://docs.rs/heck/latest/heck/index.html

