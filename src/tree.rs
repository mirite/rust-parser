use crate::types::{TagType, Token, TokenType};

/// A tree structure that represents the hierarchical structure of the HTML document.
pub struct Tree {
    pub(crate) children: Vec<Tree>,
    pub(crate) token: Token,
}

/// Adds the token to the last open tag in the tree.
fn add_to_last_open(tree: &mut Vec<Tree>, open_tags:&Vec<usize>, token:&Token) {
    if open_tags.len() == 0 {
        tree.push(Tree {
            children: vec![],
            token: token.clone(),
        })
    } else {
    let last_open = open_tags.last().unwrap();
    tree[*last_open].children.push(Tree {
        children: vec![],
        token: token.clone(),
    });
    }
}

/// Create a hierarchical tree from the tokens.
pub fn build_tree(tokens: &Vec<Token>) -> Vec<Tree> {
    let mut tree: Vec<Tree> = vec![];
    let mut open_tags: Vec<usize> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        match token.token_type {
            TokenType::Tag => match token.tag_type {
                Some(TagType::Comment) => {
                    add_to_last_open(&mut tree, &open_tags, token);
                }
                Some(TagType::Void) => {
                    add_to_last_open(&mut tree, &open_tags, token);
                }
                Some(TagType::Open) => {
                    add_to_last_open(&mut tree, &open_tags, token);
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
