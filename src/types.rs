use std::cmp::PartialEq;

#[derive(PartialEq, Debug)]
pub enum TagType {
    Open,
    Close,
    Void,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Tag,
    Text,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub(crate) name: String,
    pub(crate) tag_type: Option<TagType>,
    pub(crate) attributes: Option<String>,
    pub(crate) token_type: TokenType,
    pub(crate) content: Option<String>,
}
