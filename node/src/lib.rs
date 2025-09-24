pub trait FromNode: Sized {
    fn from_node(node: &Node) -> Result<Self, Error>;
}

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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    RequiredError(String),
    #[error("JSONの値の変換に失敗しました（{0}）")]
    ConversionError(String),
}
