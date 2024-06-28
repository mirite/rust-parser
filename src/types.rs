use std::cmp::PartialEq;

#[derive(PartialEq, Debug, Clone)]
pub enum TagType {
    Open,
    Close,
    Void,
    Comment,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Tag,
    Text,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub(crate) name: Option<String>,
    pub(crate) tag_type: Option<TagType>,
    pub(crate) attributes: Option<String>,
    pub(crate) token_type: TokenType,
    pub(crate) content: Option<String>,
}
