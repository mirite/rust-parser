use crate::types::{TagType, Token, TokenType};

pub struct Tree {
    pub(crate) children: Vec<Tree>,
    pub(crate) token: Token,
}

pub fn build_tree(tokens: &Vec<Token>) -> Vec<&Tree> {
    let mut tree: Vec<&Tree> = vec![];
    let mut open_tags: Vec<usize> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        match token.token_type {
            TokenType::Tag => match token.tag_type {
                Some(TagType::Comment) => {
                    // Add the node as a child of the most recent open.
                }
                Some(TagType::Void) => {
                    // Add the node as a child of the most recent open.
                }
                Some(TagType::Open) => {
                    let new_tree = Tree {
                        children: vec![],
                        token: token.clone(),
                    };

                    if open_tags.len() == 0 {
                        tree.push(&new_tree)
                    }
                    open_tags.push(index);
                }
                Some(TagType::Close) => {
                    // Remove the last open node of the same type from the list.
                }
                None => panic!("Tag token missing type"),
            },
            TokenType::Text => {
                // Add the node as a child of the most recent open.
            }
        }
        if token.tag_type == Some(TagType::Open) {}
        index += 1;
    }

    tree
}
