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

pub struct Token {
    pub(crate) name: String,
    pub(crate) tag_type: Option<TagType>,
    pub(crate) attributes: Option<String>,
    pub(crate) token_type: TokenType,
    pub(crate) content: Option<String>,
}

#[derive(PartialEq, Debug)]
enum Mode {
    Tag,
    Text,
}

const VOID_TAGS: [&str; 10] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
];

fn create_text_node(buffer: &Vec<char>) -> Token {
    Token {
        name: String::from("text"),
        token_type: TokenType::Text,
        content: Some(buffer.iter().collect()),
        tag_type: None,
        attributes: None,
    }
}

fn get_tag_name(buffer: &Vec<char>, first_space: Option<usize>) -> String {
    let first_char = buffer[1];
    let last_char = buffer[buffer.len() - 2];

    let slice_start = if first_char == '/' { 2 } else { 1 };
    let slice_end = if last_char == '/' {
        buffer.len() - 2
    } else {
        buffer.len() - 1
    };

    let name: String = if let Some(space) = first_space {
        buffer[slice_start..space].iter().collect()
    } else {
        buffer[slice_start..slice_end].iter().collect()
    };
    String::from(name.trim())
}

fn create_tag_node(buffer: &Vec<char>) -> Token {
    let first_space = buffer.iter().position(|&x| x == ' ');
    let tag_name = get_tag_name(&buffer, first_space);

    let tag_type = if buffer[1] == '/' {
        TagType::Close
    } else if VOID_TAGS.contains(&tag_name.as_str()) {
        TagType::Void
    } else {
        TagType::Open
    };

    let attributes = if let Some(space) = first_space {
        let val: Option<String> = Some(buffer[space + 1..buffer.len() - 1].iter().collect());
        Some(String::from(val.unwrap().trim()))
    } else {
        None
    };
    Token {
        name: tag_name,
        tag_type: Some(tag_type),
        attributes,
        token_type: TokenType::Tag,
        content: None,
    }
}

fn is_tag_close(buffer: &Vec<char>) -> bool {
    (buffer[0] == '<' && buffer[buffer.len() - 1] == '>')
        || (buffer[0] == '[' && buffer[buffer.len() - 1] == ']')
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut mode = Mode::Text;

    // Current tag tracking
    let mut buffer: Vec<char> = vec![];

    let mut index = 0;
    let mut tokens = vec![];
    while index < input.len() {
        let c = input.chars().nth(index).unwrap();
        if mode == Mode::Text {
            if c == '<' || c == '[' {
                mode = Mode::Tag;
                if buffer.len() > 0 {
                    tokens.push(create_text_node(&buffer));
                    buffer.clear();
                }
            }
        } else if mode == Mode::Tag {
            if is_tag_close(&buffer) {
                mode = Mode::Text;
                tokens.push(create_tag_node(&buffer));
                buffer.clear();
            }
        }

        buffer.push(input.chars().nth(index).unwrap());

        index += 1;
    }

    if buffer.len() > 0 {
        if is_tag_close(&buffer) {
            tokens.push(create_tag_node(&buffer));
        } else {
            tokens.push(create_text_node(&buffer));
        }
    }
    tokens
}
