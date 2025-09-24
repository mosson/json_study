use std::collections::BTreeMap;

use macro_deserialize::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Foo {
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
    true_value: bool,
    false_value: bool,
    string: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Bar {
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
    true_value: Option<bool>,
    false_value: Option<bool>,
    string: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object = node::Node::Object(BTreeMap::from([
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
        ("true_value".into(), node::Node::True),
        ("false_value".into(), node::Node::False),
        ("string".into(), node::Node::String("Hello, World!".into())),
    ]));

    let foo = Foo::from_value(&object)?;

    println!("{:#?}", foo);

    let bar = Bar::from_value(&object)?;

    println!("{:#?}", bar);

    let bar = Bar::from_value(&node::Node::Object(BTreeMap::new()))?;

    println!("{:#?}", bar);

    let object = node::Node::Object(BTreeMap::from([
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
        ("true_value".into(), node::Node::Null),
        ("false_value".into(), node::Node::Null),
        ("string".into(), node::Node::Null),
    ]));

    let bar = Bar::from_value(&object)?;

    println!("{:#?}", bar);

    let foo_result = Foo::from_value(&object);

    println!("{:#?}", foo_result);

    Ok(())
}
