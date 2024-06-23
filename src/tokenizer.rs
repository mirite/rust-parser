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

pub struct Token {
    pub(crate) name: String,
    pub(crate) tag_type: TagType,
    pub(crate) attributes: Option<String>,
    pub(crate) token_type: TokenType,
    pub(crate) content: Option<String>,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let new_token = Token {
        name: String::from("div"),
        tag_type: TagType::Open,
        attributes: None,
        token_type: TokenType::Tag,
        content: None,
    };
    vec![new_token]
}
