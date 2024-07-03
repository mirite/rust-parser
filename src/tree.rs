use crate::types::{TagType, Token, TokenType};

/// A tree structure that represents the hierarchical structure of the HTML document.
pub struct Tree {
    pub(crate) children: Vec<Tree>,
    pub(crate) token: Token,
}

/// Adds the token to the last open tag in the tree.
fn add_to_last_open(tree: &mut Vec<Tree>, open_tags: &mut Vec<&Tree>, token: &Token) -> &Tree {
    let node = Tree {
        children: vec![],
        token: token.clone(),
    };
    if open_tags.len() == 0 {
        tree.push(node);
    } else {
        let last_open = open_tags.last().unwrap();
        last_open.children.push(node);
    }
    &node;
}

/// Create a hierarchical tree from the tokens.
pub fn build_tree(tokens: &Vec<Token>) -> Vec<Tree> {
    let mut tree: Vec<Tree> = vec![];
    let mut open_tags: Vec<&Tree> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        match token.token_type {
            TokenType::Tag => match token.tag_type {
                Some(TagType::Comment) => {
                    add_to_last_open(&mut tree, &mut open_tags, token);
                }
                Some(TagType::Void) => {
                    add_to_last_open(&mut tree, &mut open_tags, token);
                }
                Some(TagType::Open) => {
                    let new_tree = add_to_last_open(&mut tree, &mut open_tags, token);
                    open_tags.push(new_tree.unwrap());
                }
                Some(TagType::Close) => {
                    let mut i = open_tags.len() - 1;
                    for open_tag in open_tags.iter().rev() {
                        if tokens[*open_tag].name == token.name {
                            open_tags.remove(i);
                            break;
                        }
                        i -= 1;
                    }
                }
                None => panic!("Tag token missing type"),
            },
            TokenType::Text => {
                add_to_last_open(&mut tree, &mut open_tags, token);
            }
        }
        index += 1;
    }

    tree
}
