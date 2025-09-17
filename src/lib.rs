/// std::io::BufRead から UTF-8 を１文字ずつ取り出すReader
pub mod char_reader;
/// char_reader::CharReader から　JSONトークンを生成する
pub mod lexer;

use crate::lexer::{Data, Lexer, Token};

/// JSONデータを表現する
#[derive(std::fmt::Debug, Clone, PartialEq)]
pub enum Node {
    String(String),
    Number(f64),
    True,
    False,
    Null,
    Array(Vec<Node>),
    Object(std::collections::BTreeMap<String, Node>),
    EOF,
}

/// 解析時のエラーを表現する
#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Error {
    #[error("行: {0:?} 位置: {1:?} で構文エラーが発生しました（{2}）")]
    SyntaxError(std::ops::Range<usize>, std::ops::Range<usize>, String),
    #[error("{0}")]
    LexerError(String),
}

impl From<lexer::error::Error> for Error {
    fn from(value: lexer::error::Error) -> Self {
        Self::LexerError(value.to_string())
    }
}

/// std::io::BufRead から読み取れる文字列からJSONデータを構築する
///
/// # Examples
///
/// ```
/// let input = r#"{"key": "Hello, 世界"}"#;
/// let cursor = std::io::Cursor::new(input);
/// let buf_reader = std::io::BufReader::new(cursor);
/// let mut parser = json_study::Parser::new(buf_reader);
/// let result = parser.parse();
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(
///     result,
///     json_study::Node::Object(
///         std::collections::BTreeMap::from([
///             (
///                 "key".to_string(),
///                 json_study::Node::String("Hello, 世界".to_string())
///             )
///         ])
///     )
/// )
/// ```
pub struct Parser<T>
where
    T: std::io::BufRead + std::fmt::Debug,
{
    lexer: Lexer<T>,
    line: std::ops::Range<usize>,
    pos: std::ops::Range<usize>,
}

#[allow(dead_code)]
impl<T> Parser<T>
where
    T: std::io::BufRead + std::fmt::Debug,
{
    /// パーサーを生成して返却する
    pub fn new(reader: T) -> Self {
        Self {
            lexer: Lexer::new(reader),
            line: 1..1,
            pos: 1..1,
        }
    }

    /// std::io::BufRead から１文字ずつ読み出し、トークンを生成し、文法からノードを構築して返却する
    /// std::io::BufRead の末尾に到達した場合は Node::EOF を返却する
    /// 構文エラーの場合は Error::SyntaxError を返却する
    /// トークン生成や reader 自体のエラーは　Error::LexerError を返却する
    pub fn parse(&mut self) -> Result<Node, Error> {
        match self.read_token()? {
            Token {
                line: _,
                pos: _,
                data: Data::LeftBrace,
            } => self.parse_object(),
            Token {
                line: _,
                pos: _,
                data: Data::LeftBracket,
            } => self.parse_array(),
            Token {
                line: _,
                pos: _,
                data: Data::String(value),
            } => Ok(Node::String(value.clone())),
            Token {
                line: _,
                pos: _,
                data: Data::Number(value),
            } => Ok(Node::Number(value.clone())),
            Token {
                line: _,
                pos: _,
                data: Data::True,
            } => Ok(Node::True),
            Token {
                line: _,
                pos: _,
                data: Data::False,
            } => Ok(Node::False),
            Token {
                line: _,
                pos: _,
                data: Data::Null,
            } => Ok(Node::Null),
            Token {
                line: _,
                pos: _,
                data: Data::EOF,
            } => Ok(Node::EOF),
            _ => return Err(self.syntax_error(
                "bool型・null型・String型・Number型・Object・Arrayのいずれかでなければなりません",
            )),
        }
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        self.lexer
            .read()
            .map(|mut token| {
                // token の line/pos を以降で読み出さない
                self.line = std::mem::take(&mut token.line);
                self.pos = std::mem::take(&mut token.pos);
                token
            })
            .map_err(Error::from)
    }

    fn parse_object(&mut self) -> Result<Node, Error> {
        let mut object = std::collections::BTreeMap::new();

        loop {
            let key_token = self.read_token()?;

            match key_token {
                Token {
                    line: _,
                    pos: _,
                    data: Data::String(key),
                } => {
                    let colon_token = self.read_token()?;

                    match colon_token {
                        Token {
                            line: _,
                            pos: _,
                            data: Data::Colon,
                        } => {
                            let value_node = self.parse()?;

                            match value_node {
                                Node::String(_)
                                | Node::Number(_)
                                | Node::True
                                | Node::False
                                | Node::Null
                                | Node::Object(_)
                                | Node::Array(_) => {
                                    match object.entry(key) {
                                        std::collections::btree_map::Entry::Occupied(mut e) => {
                                            *e.get_mut() = value_node;
                                        }
                                        std::collections::btree_map::Entry::Vacant(e) => {
                                            e.insert(value_node);
                                        }
                                    };

                                    match self.read_token()? {
                                        Token {
                                            line: _,
                                            pos: _,
                                            data: Data::Comma,
                                        } => continue,
                                        Token {
                                            line: _,
                                            pos: _,
                                            data: Data::RightBrace,
                                        } => break,
                                        _ => return Err(self.syntax_error("Objectの解析の継続（`,`）、終了（`}`）のいずれもでありません")),
                                    }
                                },
                                _ => return Err(self.syntax_error("Objectの値はbool型・null型・String型・Number型・Object・Arrayのいずれかでなければなりません")),
                            }
                        }
                        _ => {
                            return Err(
                                self.syntax_error("Objectのキーの後は`:`でなければなりません")
                            );
                        }
                    }
                }

                _ => return Err(self.syntax_error("ObjectのキーはString型でなければなりません")),
            }
        }

        Ok(Node::Object(object))
    }

    fn parse_array(&mut self) -> Result<Node, Error> {
        let mut array: Vec<Node> = Vec::new();

        loop {
            let node = self.parse()?;

            match node {
                Node::String(_)
                | Node::Number(_)
                | Node::True
                | Node::False
                | Node::Null
                | Node::Object(_)
                | Node::Array(_) => array.push(node),
                _ => return Err(self.syntax_error("Arrayの要素はbool型・null型・String型・Number型・Object・Arrayのいずれかでなければなりません")),
            }

            match self.read_token()? {
                Token {
                    line: _,
                    pos: _,
                    data: Data::Comma,
                } => continue,
                Token {
                    line: _,
                    pos: _,
                    data: Data::RightBracket,
                } => break,
                _ => {
                    return Err(
                        self.syntax_error("Arrayの要素の後は `,` か `]` でなければなりません")
                    );
                }
            }
        }

        Ok(Node::Array(array))
    }

    fn syntax_error(&self, message: &str) -> Error {
        Error::SyntaxError(self.line.clone(), self.pos.clone(), message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
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

        let result = parser.parse();
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(
            result,
            Node::Object(std::collections::BTreeMap::from([
                (
                    "array".to_string(),
                    Node::Array(vec![
                        Node::String("text".into()),
                        Node::Number(123.0),
                        Node::False,
                        Node::Null,
                        Node::Object(std::collections::BTreeMap::from([(
                            "nested_key".to_string(),
                            Node::String("nested_value".to_string())
                        )]))
                    ])
                ),
                ("boolean_false".to_string(), Node::False),
                ("boolean_true".to_string(), Node::True),
                ("null_value".to_string(), Node::Null),
                ("number_exponent".to_string(), Node::Number(12300.0)),
                ("number_float".to_string(), Node::Number(3.14159)),
                ("number_integer".to_string(), Node::Number(42.0)),
                ("number_negative".to_string(), Node::Number(-123.0)),
                (
                    "object".to_string(),
                    Node::Object(std::collections::BTreeMap::from([
                        ("key1".to_string(), Node::String("value1".into())),
                        ("key2".to_string(), Node::Number(2.0)),
                        ("key3".to_string(), Node::True),
                    ]))
                ),
                ("string".to_string(), Node::String("Hello, 世界".into())),
            ]))
        );

        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Node::EOF);
    }

    #[rstest::rstest]
    #[case("{", "ObjectのキーはString型でなければなりません")]
    #[case(
        "[",
        "Arrayの要素はbool型・null型・String型・Number型・Object・Arrayのいずれかでなければなりません"
    )]
    #[case("\"hello", "文字列の終了の前に末尾に到達しました")]
    #[case(
        r#"{"hello"; "world"}"#,
        "構文エラーが発生しました（Objectのキーの後は`:`でなければなりません）"
    )]
    #[case(
        r#"{"hello", "world"}"#,
        "構文エラーが発生しました（Objectのキーの後は`:`でなければなりません）"
    )]
    #[case(
        r#"{"hello": "world",,,,}"#,
        "構文エラーが発生しました（ObjectのキーはString型でなければなりません）"
    )]
    #[case(
        r#"{hello: "world"}"#,
        "構文エラーが発生しました（ObjectのキーはString型でなければなりません）"
    )]
    #[case(r#"{"hello": truthy}"#, "`true` トークンの解釈に失敗しました")]
    fn test_syntax_error(#[case] input: &str, #[case] message: &str) {
        let cursor = std::io::Cursor::new(input);
        let buf_reader = std::io::BufReader::new(cursor);
        let mut parser = Parser::new(buf_reader);

        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains(message));
    }
}
