#[cfg(test)]
mod tests {
    use crate::tree::build_tree;
    use crate::types;

    #[test]
    fn it_can_handle_a_simple_tree() {
        let tokens = vec![
            types::Token {
                name: Some("div".to_string()),
                tag_type: Some(types::TagType::Open),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
            types::Token {
                name: Some("text".to_string()),
                tag_type: None,
                attributes: None,
                content: Some("Hello".to_string()),
                token_type: types::TokenType::Text,
            },
            types::Token {
                name: Some("div".to_string()),
                tag_type: Some(types::TagType::Close),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];

        let tree = build_tree(&tokens);
        assert_eq!(tree.len(), 1);
    }
}
