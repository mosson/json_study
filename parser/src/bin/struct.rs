use macro_deserialize::Deserialize;

#[derive(Deserialize)]
struct Foo {
    bar: String,
    baz: usize,
}

fn main() {}
