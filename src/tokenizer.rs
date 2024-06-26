use crate::types::{TagType, Token, TokenType};

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
        name: Some(String::from("text")),
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
    } else if buffer[1] == '!' && buffer[2] == '-' && buffer[3] == '-' {
        TagType::Comment
    } else if VOID_TAGS.contains(&tag_name.as_str()) {
        TagType::Void
    } else {
        TagType::Open
    };

    let attributes = if tag_type == TagType::Comment {
        None
    } else if let Some(space) = first_space {
        let val: Option<String> = Some(buffer[space + 1..buffer.len() - 1].iter().collect());
        Some(String::from(val.unwrap().trim()))
    } else {
        None
    };

    let content = if tag_type == TagType::Comment {
        Some(buffer[4..buffer.len() - 3].iter().collect())
    } else {
        None
    };

    Token {
        name: match tag_type {
            TagType::Comment => None,
            _ => Some(tag_name),
        },
        tag_type: Some(tag_type),
        attributes,
        token_type: TokenType::Tag,
        content,
    }
}

fn is_tag_close(buffer: &Vec<char>, last_char: char) -> bool {
    let first_char = buffer[0];
    (first_char == '<' && last_char == '>') || (first_char == '[' && last_char == ']')
}

fn is_tag_open(character: char) -> bool {
    character == '<' || character == '['
}

fn merge_text_nodes(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut index = 0;
    while index < tokens.len() {
        if tokens[index].token_type == TokenType::Text {
            let mut text = tokens[index].content.clone().unwrap();
            let next_index = index + 1;
            while next_index < tokens.len() && tokens[next_index].token_type == TokenType::Text {
                text.push_str(&tokens[next_index].content.clone().unwrap());
                tokens.remove(next_index);
            }
            tokens[index].content = Some(text);
        }
        index += 1;
    }
    tokens
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut mode = Mode::Text;

    // Current tag tracking
    let mut buffer: Vec<char> = vec![];

    let mut index = 0;
    let mut tokens = vec![];
    while index < input.len() {
        let ch = input.chars().nth(index).unwrap();
        if mode == Mode::Text {
            if is_tag_open(ch) {
                mode = Mode::Tag;
                if buffer.len() > 0 {
                    tokens.push(create_text_node(&buffer));
                    buffer.clear();
                }
            }
        } else if mode == Mode::Tag {
            // // A tag was opened, but a new tag was started. Treat the previous tag as text.
            if is_tag_open(ch) {
                mode = Mode::Text;
                if buffer.len() > 0 {
                    tokens.push(create_text_node(&buffer));
                    buffer.clear();
                }
            } else if is_tag_close(&buffer, ch) {
                mode = Mode::Text;
                buffer.push(ch);

                index += 1;
                tokens.push(create_tag_node(&buffer));
                buffer.clear();
            }
        }

        if index < input.len() {
            buffer.push(input.chars().nth(index).unwrap());
        }

        index += 1;
    }

    if buffer.len() > 0 {
        if is_tag_close(&buffer, buffer[buffer.len() - 1]) {
            tokens.push(create_tag_node(&buffer));
        } else {
            tokens.push(create_text_node(&buffer));
        }
    }

    merge_text_nodes(tokens)
}
