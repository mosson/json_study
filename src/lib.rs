/// std::io::BufRead から UTF-8 を１文字ずつ取り出すReader
pub mod char_reader;
/// char_reader::CharReader から　JSONトークンを生成する
pub mod lexer;
