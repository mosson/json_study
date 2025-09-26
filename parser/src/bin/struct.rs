use macro_deserialize::Deserialize;
use node::FromNode;
use parser::Parser;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Foo {
        string: String,
        i8: i8,
        i16: i16,
        i32: i32,
        i64: i64,
        isize: isize,
        u8: u8,
        u16: u16,
        u32: u32,
        u64: u64,
        usize: usize,
        f64: f64,
        true_value: bool,
        false_value: bool,
    }

    let object = node::Node::Object(BTreeMap::from([
        ("string".into(), node::Node::String("Hello, World!".into())),
        ("i8".into(), node::Node::Number(-10f64)),
        ("i16".into(), node::Node::Number(-20f64)),
        ("i32".into(), node::Node::Number(-30f64)),
        ("i64".into(), node::Node::Number(-40f64)),
        ("isize".into(), node::Node::Number(-50f64)),
        ("u8".into(), node::Node::Number(10f64)),
        ("u16".into(), node::Node::Number(20f64)),
        ("u32".into(), node::Node::Number(30f64)),
        ("u64".into(), node::Node::Number(40f64)),
        ("usize".into(), node::Node::Number(50f64)),
        ("f64".into(), node::Node::Number(60.123f64)),
        ("true_value".into(), node::Node::True),
        ("false_value".into(), node::Node::False),
    ]));

    let foo = Foo::from_node(&object)?;

    println!("{:#?}", foo);

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Bar {
        string: Option<String>,
        i8: Option<i8>,
        i16: Option<i16>,
        i32: Option<i32>,
        i64: Option<i64>,
        isize: Option<isize>,
        u8: Option<u8>,
        u16: Option<u16>,
        u32: Option<u32>,
        u64: Option<u64>,
        usize: Option<usize>,
        f64: Option<f64>,
        true_value: Option<bool>,
        false_value: Option<bool>,
    }

    let bar = Bar::from_node(&object)?;

    println!("{:#?}", bar);

    let bar = Bar::from_node(&node::Node::Object(BTreeMap::new()))?;

    println!("{:#?}", bar);

    let object = node::Node::Object(BTreeMap::from([
        ("string".into(), node::Node::Null),
        ("i8".into(), node::Node::Null),
        ("i16".into(), node::Node::Null),
        ("i32".into(), node::Node::Null),
        ("i64".into(), node::Node::Null),
        ("isize".into(), node::Node::Null),
        ("u8".into(), node::Node::Null),
        ("u16".into(), node::Node::Null),
        ("u32".into(), node::Node::Null),
        ("u64".into(), node::Node::Null),
        ("usize".into(), node::Node::Null),
        ("f64".into(), node::Node::Null),
        ("true_value".into(), node::Node::Null),
        ("false_value".into(), node::Node::Null),
    ]));

    let bar = Bar::from_node(&object)?;

    println!("{:#?}", bar);

    let foo_result = Foo::from_node(&object);

    println!("{:#?}", foo_result);

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct A {
        b: B,
        optional_b: Option<B>,
        optional_b_2: Option<B>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct B {
        c: usize,
        d: Option<usize>,
    }

    let object = node::Node::Object(BTreeMap::from([
        (
            "b".into(),
            node::Node::Object(BTreeMap::from([
                ("c".into(), node::Node::Number(12f64)),
                ("d".into(), node::Node::Null),
            ])),
        ),
        (
            "optional_b".into(),
            node::Node::Object(BTreeMap::from([
                ("c".into(), node::Node::Number(12f64)),
                ("d".into(), node::Node::Null),
            ])),
        ),
        ("optional_b_2".into(), node::Node::Null),
    ]));

    let a = A::from_node(&object);

    println!("{:#?}", a);

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Vector {
        usize: Vec<usize>,
        nested_string: Vec<Vec<String>>,
        optional_f64: Vec<Option<f64>>,
        deep_nested_i16: Vec<InnerVector>,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct InnerVector {
        v: Vec<i16>,
    }

    let object = node::Node::Object(BTreeMap::from([
        (
            "usize".into(),
            node::Node::Array(vec![
                node::Node::Number(10f64),
                node::Node::Number(11f64),
                node::Node::Number(22f64),
            ]),
        ),
        (
            "nested_string".into(),
            node::Node::Array(vec![
                node::Node::Array(vec![
                    node::Node::String("a".into()),
                    node::Node::String("b".into()),
                    node::Node::String("c".into()),
                ]),
                node::Node::Array(vec![
                    node::Node::String("d".into()),
                    node::Node::String("e".into()),
                    node::Node::String("f".into()),
                ]),
            ]),
        ),
        (
            "optional_f64".into(),
            node::Node::Array(vec![
                node::Node::Number(10f64),
                node::Node::Null,
                node::Node::Number(22f64),
            ]),
        ),
        (
            "deep_nested_i16".into(),
            node::Node::Array(vec![
                node::Node::Object(BTreeMap::from([(
                    "v".into(),
                    node::Node::Array(vec![node::Node::Number(-10f64), node::Node::Number(22f64)]),
                )])),
                node::Node::Object(BTreeMap::from([(
                    "v".into(),
                    node::Node::Array(vec![node::Node::Number(-66f64), node::Node::Number(77f64)]),
                )])),
            ]),
        ),
    ]));

    let vector = Vector::from_node(&object);

    println!("{:#?}", vector);

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct DeriveTuple {
        t: (usize, String, i16),
        t2: (Option<usize>, String, Option<i16>),
        t3: Option<(Option<usize>, String, Option<i16>)>,
    }

    let object = node::Node::Object(BTreeMap::from([
        (
            "t".into(),
            node::Node::Array(vec![
                node::Node::Number(4f64),
                node::Node::String("Hello, World!".into()),
                node::Node::Number(-8f64),
            ]),
        ),
        (
            "t2".into(),
            node::Node::Array(vec![
                node::Node::Number(4f64),
                node::Node::String("Hello, World!".into()),
                node::Node::Null,
            ]),
        ),
        ("t3".into(), node::Node::Null),
    ]));

    let derive_tuple = DeriveTuple::from_node(&object);

    println!("{:#?}", derive_tuple);

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Full {
        string: String,
        number_integer: usize,
        number_negative: isize,
        number_float: f64,
        number_exponent: f64,
        boolean_true: bool,
        boolean_false: bool,
        null_value: Option<usize>,
        array: (String, usize, bool, Option<usize>, FullNested),
        object: FullNested2,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct FullNested {
        nested_key: String,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct FullNested2 {
        key1: String,
        key2: usize,
        key3: bool,
    }

    let input = r#"
        {
            "string": "Hello, 世界",
            "number_integer": 42,
            "number_negative": -123,
            "number_float": 3.14159,
            "number_exponent": 1.23e4,
            "boolean_true": true,
            "boolean_false": false,
            "null_value": null,
            "array": [
                "text",
                123,
                false,
                null,
                {
                "nested_key": "nested_value"
                }
            ],
            "object": {
                "key1": "value1",
                "key2": 2,
                "key3": true
            }
        }
    "#;

    let cursor = std::io::Cursor::new(input);
    let buf_reader = std::io::BufReader::new(cursor);
    let mut parser = Parser::new(buf_reader);

    let node = parser.parse();
    assert!(node.is_ok());

    let parsed = Full::from_node(&node.unwrap());

    println!("{:#?}", parsed);

    Ok(())
}
