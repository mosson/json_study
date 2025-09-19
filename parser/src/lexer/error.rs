use crate::char_reader;

/// トークン生成時のエラーを表現する
#[derive(thiserror::Error, std::fmt::Debug, PartialEq)]
pub enum Error {
    #[error("")]
    EOF(usize, usize),
    #[error("Line: {0:?} Position: {1:?} 文字列の終了の前に末尾に到達しました")]
    UnclosedStringLiteral(std::ops::Range<usize>, std::ops::Range<usize>),
    #[error("{0}")]
    ReaderError(String),
    #[error("Line: {1:?} Position: {2:?} `{0}` トークンの解釈に失敗しました")]
    InvalidToken(String, std::ops::Range<usize>, std::ops::Range<usize>),
    #[error("Line: {1:?} Position: {2:?} `number` トークンとして解釈できませんでした（{0}） ")]
    InvalidNumber(String, std::ops::Range<usize>, std::ops::Range<usize>),
}

impl From<char_reader::error::Error> for Error {
    fn from(value: char_reader::error::Error) -> Self {
        Self::ReaderError(value.to_string())
    }
}
