use macro_deserialize::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Foo {
    bar: String,
    baz: Option<String>,
}

fn main() {}
