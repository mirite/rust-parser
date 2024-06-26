#[cfg(test)]
mod tests {
    use crate::{tokenizer, types};

    #[test]
    fn simple_string() {
        let input = "<div>Hello</div><img/>";
        let expected = vec![
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
            types::Token {
                name: Some("img".to_string()),
                tag_type: Some(types::TagType::Void),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];

        check_result(input, expected);
    }

    fn check_result(input: &str, expected: Vec<types::Token>) {
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
            types::Token {
                name: Some("div".to_string()),
                tag_type: Some(types::TagType::Open),
                attributes: Some("data-testid='bonjour' class='w-4'".to_string()),
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
            types::Token {
                name: Some("img".to_string()),
                tag_type: Some(types::TagType::Void),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn massive_whitespace() {
        let input = "<span\n                                                                                style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\">of\n                                                        liquid</span>";

        let expected = vec![
          types::Token {
              name: Some("span".to_string()),
              tag_type: Some(types::TagType::Open),
              attributes: Some("style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\"".to_string()),
              content: None,
              token_type: types::TokenType::Tag,
          },
            types::Token {
                name: Some("text".to_string()),
                tag_type: None,
                attributes: None,
                content: Some("of\n                                                        liquid".to_string()),
                token_type: types::TokenType::Text,
            },
            types::Token {
                name: Some("span".to_string()),
                tag_type: Some(types::TagType::Close),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn mixed_format() {
        let input = "[et_pb_section]Hello<br>World[/et_pb_section]";

        let expected = vec![
            types::Token {
                name: Some("et_pb_section".to_string()),
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
                name: Some("br".to_string()),
                tag_type: Some(types::TagType::Void),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
            types::Token {
                name: Some("text".to_string()),
                tag_type: None,
                attributes: None,
                content: Some("World".to_string()),
                token_type: types::TokenType::Text,
            },
            types::Token {
                name: Some("et_pb_section".to_string()),
                tag_type: Some(types::TagType::Close),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn unmatched_tags() {
        let input = "[et_pb_section]Two<One[/et_pb_section]";
        let expected = vec![
            types::Token {
                name: Some("et_pb_section".to_string()),
                tag_type: Some(types::TagType::Open),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
            types::Token {
                name: Some("text".to_string()),
                tag_type: None,
                attributes: None,
                content: Some("Two<One".to_string()),
                token_type: types::TokenType::Text,
            },
            types::Token {
                name: Some("et_pb_section".to_string()),
                tag_type: Some(types::TagType::Close),
                attributes: None,
                content: None,
                token_type: types::TokenType::Tag,
            },
        ];
        check_result(input, expected);
    }

    #[test]
    fn leading_text() {
        let input = "Hello<div>World</div>";
        let expected = vec![
            types::Token {
                name: Some("text".to_string()),
                tag_type: None,
                attributes: None,
                content: Some("Hello".to_string()),
                token_type: types::TokenType::Text,
            },
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
                content: Some("World".to_string()),
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
        check_result(input, expected);
    }

    #[test]
    fn trailing_text() {
        let input = "<div>World</div>Hello";
        let expected = vec![
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
                content: Some("World".to_string()),
                token_type: types::TokenType::Text,
            },
            types::Token {
                name: Some("div".to_string()),
                tag_type: Some(types::TagType::Close),
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
        ];
        check_result(input, expected);
    }

    #[test]
    fn comments() {
        let input = "<!-- Hello --><div>World</div>";
        let expected = vec![
            types::Token {
                name: None,
                tag_type: Some(types::TagType::Comment),
                attributes: None,
                content: Some(" Hello ".to_string()),
                token_type: types::TokenType::Tag,
            },
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
                content: Some("World".to_string()),
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
        check_result(input, expected);
    }
}
