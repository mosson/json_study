use parser::Parser;

fn main() {
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

    println!("{:#?}", parser.parse());
    println!("{:#?}", parser.parse());
}
