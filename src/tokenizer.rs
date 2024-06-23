#[derive(PartialEq)]
#[derive(Debug)]
pub enum TagType {
    Open,
    Close,
    Void,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    Tag,
    Text,
}

struct Token {
    pub(crate) name: String,
    pub(crate) tag_type: TagType,
    pub(crate) attributes: Option<String>,
    pub(crate) token_type: TokenType,
    pub(crate) content: Option<String>,
}

    pub fn tokenize(input: &str) -> Vec<&Token> {
        input.split(|c| c == '<' || c == '>').collect()
    }

