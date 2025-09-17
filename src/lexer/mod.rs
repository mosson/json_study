/// トークン生成時のエラーを表現する
pub mod error;

use crate::{
    char_reader::{self, CharReader},
    lexer::error::Error,
};

/// JSONのトークンを表現する
/// トークン時点では文法の評価はしない
#[derive(std::fmt::Debug, PartialEq)]
#[allow(dead_code)]
pub struct Token {
    pub line: std::ops::Range<usize>,
    pub pos: std::ops::Range<usize>,
    pub data: Data,
}

impl Token {
    fn new(line: std::ops::Range<usize>, pos: std::ops::Range<usize>, data: Data) -> Self {
        Self { line, pos, data }
    }
}

/// トークンの種別を表す
#[derive(std::fmt::Debug, PartialEq)]
pub enum Data {
    String(String),
    Number(f64),
    True,
    False,
    Null,
    Colon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    EOF,
}

/// std::io::BufRead から UTF-8 を１文字ずつ読み取り、JSONトークンを返却する
/// 文法の評価はしないが、データ型に違反している場合はエラーを返す（数値リテラルなのに数値として解釈できない: Error::InvalidNumber）
///
/// # Examples
///
/// ```
/// let input = r#"{"key": "value"}"#;
/// let cursor = std::io::Cursor::new(input);
/// let buf_reader = std::io::BufReader::new(cursor);
/// let mut lexer = json_study::lexer::Lexer::new(buf_reader);
/// let mut tokens = vec![];    
///
/// loop {
///     let result = lexer.read();
///     assert!(result.is_ok());
///     let token = result.unwrap();
///     match token {
///         json_study::lexer::Token {
///             line: _,
///             pos: _,
///             data: json_study::lexer::Data::EOF
///         } => break,
///         _ => tokens.push(token),
///     }
/// }
///
/// assert_eq!(
///     tokens,
///     vec![
///         json_study::lexer::Token {
///             line: 1..1,
///             pos: 1..1,
///             data: json_study::lexer::Data::LeftBrace,
///         },
///         json_study::lexer::Token {
///             line: 1..1,
///             pos: 2..6,
///             data: json_study::lexer::Data::String("key".into()),
///         },
///         json_study::lexer::Token {
///             line: 1..1,
///             pos: 7..7,
///             data: json_study::lexer::Data::Colon,
///         },
///         json_study::lexer::Token {
///             line: 1..1,
///             pos: 9..15,
///             data: json_study::lexer::Data::String("value".into()),
///         },
///         json_study::lexer::Token {
///             line: 1..1,
///             pos: 16..16,
///             data: json_study::lexer::Data::RightBrace,
///         },
///     ]
/// )
/// ```
///
#[allow(dead_code)]
pub struct Lexer<T>(CharReader<T>)
where
    T: std::io::BufRead + std::fmt::Debug;

#[allow(dead_code)]
impl<T> Lexer<T>
where
    T: std::io::BufRead + std::fmt::Debug,
{
    /// トークナイザーを生成して返却する
    pub fn new(reader: T) -> Self {
        Self(CharReader::new(reader))
    }

    fn discard_next(&mut self) -> (char, usize, usize) {
        self.next().expect("peekと内容が異なる")
    }

    /// reader から複数文字読み出しトークンを生成して返却する
    /// トークン生成に失敗するか　reader からの読み出しに失敗する場合は Error を返却する
    pub fn read(&mut self) -> Result<Token, Error> {
        let peek = self.peek().cloned();

        match peek {
            Err(Error::EOF(line, pos)) => Ok(Token::new(line..line, pos..pos, Data::EOF)),
            Err(e) => Err(e),
            Ok((c, _, _)) => {
                let result = match c {
                    '"' => self.parse_string(),
                    '-' | '1'..='9' | '0' => self.parse_number(),
                    't' => self.parse_static::<'t'>(),
                    'f' => self.parse_static::<'f'>(),
                    'n' => self.parse_static::<'n'>(),
                    ':' => self.parse_delimiter::<':'>(),
                    ',' => self.parse_delimiter::<','>(),
                    '{' => self.parse_delimiter::<'{'>(),
                    '}' => self.parse_delimiter::<'}'>(),
                    '[' => self.parse_delimiter::<'['>(),
                    ']' => self.parse_delimiter::<']'>(),
                    // それ以外の文字は読み飛ばす
                    _ => {
                        // ピーク分を破棄する
                        self.discard_next();
                        // 再帰的に次のトークンの処理を呼び出す
                        self.read()
                    }
                };

                match result {
                    Err(Error::EOF(line, pos)) => Ok(Token::new(line..line, pos..pos, Data::EOF)),
                    Err(e) => return Err(e),
                    Ok(token) => Ok(token),
                }
            }
        }
    }

    fn next(&mut self) -> Result<(char, usize, usize), Error> {
        self.0.read().map_err(|e| match e {
            char_reader::error::Error::EOF(line, pos) => Error::EOF(line, pos),
            _ => Error::from(e),
        })
    }

    fn peek(&mut self) -> Result<&(char, usize, usize), Error> {
        self.0.peek().map_err(|e| match e {
            char_reader::error::Error::EOF(line, pos) => Error::EOF(line, pos),
            _ => Error::from(e),
        })
    }

    fn peek_back(&mut self) -> Result<(), Error> {
        self.0.peek_back().map_err(Error::from)
    }

    fn parse_string(&mut self) -> Result<Token, Error> {
        let mut buf = Vec::new();

        // トークン開始位置のダブルクォートを読み捨て
        let (_, initial_line, initial_pos) = self.discard_next();
        let final_line: usize;
        let final_pos: usize;

        loop {
            let (c, _, _) = self.peek().map_err(|e| match e {
                Error::EOF(line, pos) => {
                    Error::UnclosedStringLiteral(initial_line..line, initial_pos..pos)
                }
                _ => e,
            })?;

            match c {
                '"' => {
                    // トークン終了位置のダブルクォートを読み捨て
                    let (_, line, pos) = self.discard_next();
                    final_line = line;
                    final_pos = pos;
                    break;
                }
                '\\' => {
                    // バッククォート読み捨て
                    self.discard_next();

                    // match の評価をせずに１文字読み込む
                    let result = self.next();

                    if let Err(Error::EOF(line, pos)) = result {
                        return Err(Error::UnclosedStringLiteral(
                            initial_line..line,
                            initial_pos..pos,
                        ));
                    }

                    buf.push(result?.0);
                }
                _ => {
                    buf.push(self.next().expect("peekと内容が異なる").0);
                }
            }
        }

        Ok(Token::new(
            initial_line..final_line,
            initial_pos..final_pos,
            Data::String(buf.into_iter().collect::<String>()),
        ))
    }

    fn parse_number(&mut self) -> Result<Token, Error> {
        let mut buf = Vec::new();
        let (c, initial_line, initial_position) = self.next().expect("peekと内容が異なる");
        let mut final_line = initial_line;
        let mut final_position = initial_position;

        buf.push(c);

        loop {
            let result = self.peek();

            if let Err(Error::EOF(_, _)) = result {
                // 次のreadでEOFトークンの返却を期待する
                break;
            }

            let (c, _, _) = result?;

            match c {
                '-' | '1'..='9' | '0' | '.' | 'e' | 'E' => {
                    let (c, line, pos) = self.next().expect("peekと内容が異なる");
                    final_line = line;
                    final_position = pos;
                    buf.push(c);
                }
                _ => break self.peek_back()?,
            }
        }

        buf.into_iter()
            .collect::<String>()
            .parse::<f64>()
            .map_err(|e| {
                Error::InvalidNumber(
                    e.to_string(),
                    initial_line..final_line,
                    initial_position..final_position,
                )
            })
            .map(|f| {
                Token::new(
                    initial_line..final_line,
                    initial_position..final_position,
                    Data::Number(f),
                )
            })
    }

    fn parse_static<const K: char>(&mut self) -> Result<Token, Error> {
        let (_, initial_line, initial_position) = self.next()?;

        let (source, data, type_name) = match K {
            't' => (vec!['r', 'u', 'e'], Data::True, "true"),
            'f' => (vec!['a', 'l', 's', 'e'], Data::False, "false"),
            'n' => (vec!['u', 'l', 'l'], Data::Null, "null"),
            _ => unreachable!("呼び出し元で規定以外の文字を処理しようとしている"),
        };

        for c in source.iter() {
            let (tc, line, position) = self.peek()?;

            if c != tc {
                return Err(Error::InvalidToken(
                    type_name.into(),
                    initial_line..*line,
                    initial_position..*position,
                ));
            }
        }

        self.0
            .consume(source.len())
            .map(|_| {
                Token::new(
                    initial_line..initial_line,
                    initial_position..(initial_position + source.len()),
                    data,
                )
            })
            .map_err(Error::from)
    }

    fn parse_delimiter<const C: char>(&mut self) -> Result<Token, Error> {
        let data = match C {
            ':' => Data::Colon,
            ',' => Data::Comma,
            '{' => Data::LeftBrace,
            '}' => Data::RightBrace,
            '[' => Data::LeftBracket,
            ']' => Data::RightBracket,
            _ => unreachable!("呼び出し元で規定以外の文字を処理しようとしている"),
        };

        let (_, line, pos) = self.discard_next();

        Ok(Token::new(line..line, pos..pos, data))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_lexer() {
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

        let cursor = Cursor::new(input);
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        let mut tokens: Vec<Token> = vec![];

        loop {
            let result = lexer.read();
            assert!(result.is_ok());
            let token = result.unwrap();
            match token.data {
                Data::EOF => break,
                _ => tokens.push(token),
            }
        }

        assert_eq!(
            tokens
                .into_iter()
                .map(|token| token.data)
                .collect::<Vec<_>>(),
            vec![
                Data::LeftBrace,
                Data::String("string".into()),
                Data::Colon,
                Data::String("Hello, 世界".into()),
                Data::Comma,
                Data::String("number_integer".into()),
                Data::Colon,
                Data::Number(42.0_f64),
                Data::Comma,
                Data::String("number_negative".into()),
                Data::Colon,
                Data::Number(-123.0_f64),
                Data::Comma,
                Data::String("number_float".into()),
                Data::Colon,
                Data::Number(3.14159_f64),
                Data::Comma,
                Data::String("number_exponent".into()),
                Data::Colon,
                Data::Number(12300.0_f64),
                Data::Comma,
                Data::String("boolean_true".into()),
                Data::Colon,
                Data::True,
                Data::Comma,
                Data::String("boolean_false".into()),
                Data::Colon,
                Data::False,
                Data::Comma,
                Data::String("null_value".into()),
                Data::Colon,
                Data::Null,
                Data::Comma,
                Data::String("array".into()),
                Data::Colon,
                Data::LeftBracket,
                Data::String("text".into()),
                Data::Comma,
                Data::Number(123.0_f64),
                Data::Comma,
                Data::False,
                Data::Comma,
                Data::Null,
                Data::Comma,
                Data::LeftBrace,
                Data::String("nested_key".into()),
                Data::Colon,
                Data::String("nested_value".into()),
                Data::RightBrace,
                Data::RightBracket,
                Data::Comma,
                Data::String("object".into()),
                Data::Colon,
                Data::LeftBrace,
                Data::String("key1".into()),
                Data::Colon,
                Data::String("value1".into()),
                Data::Comma,
                Data::String("key2".into()),
                Data::Colon,
                Data::Number(2.0_f64),
                Data::Comma,
                Data::String("key3".into()),
                Data::Colon,
                Data::True,
                Data::RightBrace,
                Data::RightBrace,
            ]
        );
    }

    #[rstest::rstest]
    #[case("\"boon\"", Token::new(1..1, 1..6, Data::String("boon".into())))]
    #[case(r#""\"english\"""#, Token::new(1..1, 1..13, Data::String(r#""english""#.into())))]
    fn test_parse_string(#[case] input: &str, #[case] expected: Token) {
        let cursor = Cursor::new(input);
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_string();
        assert!(result.is_ok());

        assert_eq!(result.unwrap(), expected);
    }

    #[rstest::rstest]
    #[case("123", Token::new(1..1, 1..3, Data::Number(123_f64)))] // 整数
    #[case("-123", Token::new(1..1, 1..4, Data::Number(-123_f64)))] // 負の整数
    #[case("3.14", Token::new(1..1, 1..4, Data::Number(3.14_f64)))] // 小数
    #[case("-0.01", Token::new(1..1, 1..5, Data::Number(-0.01_f64)))] // 負の小数
    #[case("1e6", Token::new(1..1, 1..3, Data::Number(1e6_f64)))] // 指数表記（10^6）
    #[case("-2.5E-3", Token::new(1..1, 1..7, Data::Number(-2.5E-3_f64)))] // 指数付き小数
    fn test_parse_number(#[case] input: &str, #[case] expected: Token) {
        let cursor = Cursor::new(input);
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_number();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest::rstest]
    #[case(
        "3.14.14",
        "Line: 1..1 Position: 1..7 `number` トークンとして解釈できませんでした（invalid float literal） "
    )]
    #[case(
        "-3E14E1",
        "Line: 1..1 Position: 1..7 `number` トークンとして解釈できませんでした（invalid float literal） "
    )]
    #[case(
        "--11223",
        "Line: 1..1 Position: 1..7 `number` トークンとして解釈できませんでした（invalid float literal） "
    )]
    fn test_parse_invalid_number(#[case] input: &str, #[case] expected: &str) {
        let cursor = Cursor::new(input);
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_number();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected);
    }

    #[rstest::rstest]
    fn test_parse_static() {
        let cursor = Cursor::new("true");
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_static::<'t'>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::new(1..1, 1..4, Data::True));

        let cursor = Cursor::new("false");
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_static::<'f'>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::new(1..1, 1..5, Data::False));

        let cursor = Cursor::new("null");
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        lexer.peek().unwrap();

        let result = lexer.parse_static::<'n'>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::new(1..1, 1..4, Data::Null));
    }

    #[test]
    fn test_unclosed_string() {
        let cursor = Cursor::new("\"true");
        let buf_reader = std::io::BufReader::new(cursor);
        let mut lexer = Lexer::new(buf_reader);

        let result = lexer.read();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::UnclosedStringLiteral(1..1, 1..5)
        )
    }
}
