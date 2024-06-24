#[cfg(test)]
mod tests {
    use crate::tokenizer;

    #[test]
    fn simple_string() {
        let input = "<div>Hello</div><img/>";
        let expected = vec![
            tokenizer::Token {
                name: "div".to_string(),
                tag_type: Some(tokenizer::TagType::Open),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("Hello".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "div".to_string(),
                tag_type: Some(tokenizer::TagType::Close),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "img".to_string(),
                tag_type: Some(tokenizer::TagType::Void),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
        ];

        check_result(input, expected);
    }

    fn check_result(input: &str, expected: Vec<tokenizer::Token>) {
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), expected.len());
        for i in 0..output.len() {
            assert_eq!(output[i], expected[i]);
        }
    }

    #[test]
    fn attributes() {
        let input = "<div data-testid='bonjour' class='w-4'>Hello</div><img>";
        let expected = vec![
            tokenizer::Token {
                name: "div".to_string(),
                tag_type: Some(tokenizer::TagType::Open),
                attributes: Some("data-testid='bonjour' class='w-4'".to_string()),
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("Hello".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "div".to_string(),
                tag_type: Some(tokenizer::TagType::Close),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "img".to_string(),
                tag_type: Some(tokenizer::TagType::Void),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn massive_whitespace() {
        let input = "<span\n                                                                                style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\">of\n                                                        liquid</span>";

        let expected = vec![
          tokenizer::Token {
              name: "span".to_string(),
              tag_type: Some(tokenizer::TagType::Open),
              attributes: Some("style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\"".to_string()),
              content: None,
              token_type: tokenizer::TokenType::Tag,
          },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("of\n                                                        liquid".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "span".to_string(),
                tag_type: Some(tokenizer::TagType::Close),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn mixed_format() {
        let input = "[et_pb_section]Hello<br>World[/et_pb_section]";

        let expected = vec![
            tokenizer::Token {
                name: "et_pb_section".to_string(),
                tag_type: Some(tokenizer::TagType::Open),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("Hello".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "br".to_string(),
                tag_type: Some(tokenizer::TagType::Void),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("World".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "et_pb_section".to_string(),
                tag_type: Some(tokenizer::TagType::Close),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn unmatched_tags() {
        let input = "[et_pb_section]Two<One[/et_pb_section]";
        let expected = vec![
            tokenizer::Token {
                name: "et_pb_section".to_string(),
                tag_type: Some(tokenizer::TagType::Open),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
            tokenizer::Token {
                name: "text".to_string(),
                tag_type: None,
                attributes: None,
                content: Some("Two<One".to_string()),
                token_type: tokenizer::TokenType::Text,
            },
            tokenizer::Token {
                name: "et_pb_section".to_string(),
                tag_type: Some(tokenizer::TagType::Close),
                attributes: None,
                content: None,
                token_type: tokenizer::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }
}
