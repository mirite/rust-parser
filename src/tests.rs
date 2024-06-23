#[cfg(test)]
mod tests {
    use crate::tokenizer;

    #[test]
    fn simple_string() {
        let input = "<div>Hello</div><img/>";
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), 4);
        assert_eq!(output[0].name, "div");
        assert_eq!(output[0].tag_type, tokenizer::TagType::Open);
        assert_eq!(output[0].attributes, None);
        assert_eq!(output[0].content, None);
        assert_eq!(output[0].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[1].name, "text");
        assert_eq!(output[1].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[1].attributes, None);
        assert_ne!(output[1].content, None);
        assert_eq!(output[1].content, Some(String::from("Hello")));

        assert_eq!(output[2].name, "div");
        assert_eq!(output[2].tag_type, tokenizer::TagType::Close);
        assert_eq!(output[2].attributes, None);
        assert_eq!(output[2].content, None);
        assert_eq!(output[2].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[3].name, "img");
        assert_eq!(output[3].tag_type, tokenizer::TagType::Void);
        assert_eq!(output[3].attributes, None);
        assert_eq!(output[3].content, None);
        assert_eq!(output[3].token_type, tokenizer::TokenType::Tag);
    }

    #[test]
    fn attributes() {
        let input = "<div data-testid='bonjour' class='w-4'>Hello</div><img>";
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), 4);
        assert_eq!(output[0].name, "div");
        assert_eq!(output[0].tag_type, tokenizer::TagType::Open);
        assert_eq!(
            output[0].attributes,
            Some(String::from("data-testid='bonjour' class='w-4'"))
        );
        assert_eq!(output[0].content, None);
        assert_eq!(output[0].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[1].name, "text");
        assert_eq!(output[1].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[1].attributes, None);
        assert_ne!(output[1].content, None);
        assert_eq!(output[1].content, Some(String::from("Hello")));

        assert_eq!(output[2].name, "div");
        assert_eq!(output[2].tag_type, tokenizer::TagType::Close);
        assert_eq!(output[2].attributes, None);
        assert_eq!(output[2].content, None);
        assert_eq!(output[2].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[3].name, "img");
        assert_eq!(output[3].tag_type, tokenizer::TagType::Void);
        assert_eq!(output[3].attributes, None);
        assert_eq!(output[3].content, None);
        assert_eq!(output[3].token_type, tokenizer::TokenType::Tag);
    }

    #[test]
    fn massive_whitespace() {
        let input = "<span\n                                                                                style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\">of\n                                                        liquid</span>";
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0].name, "span");
        assert_eq!(output[0].tag_type, tokenizer::TagType::Open);
        assert_eq!(output[0].attributes, Some(String::from("style=\"display: block; font-size: 1.5rem; font-weight: 900; text-transform: uppercase; line-height: 0.5rem;\"")));
        assert_eq!(output[0].content, None);
        assert_eq!(output[0].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[1].name, "text");
        assert_eq!(output[1].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[1].attributes, None);
        assert_ne!(output[1].content, None);
        assert_eq!(
            output[1].content,
            Some(String::from(
                "of\n                                                        liquid"
            ))
        );

        assert_eq!(output[2].name, "span");
        assert_eq!(output[2].tag_type, tokenizer::TagType::Close);
        assert_eq!(output[2].attributes, None);
        assert_eq!(output[2].content, None);
        assert_eq!(output[2].token_type, tokenizer::TokenType::Tag);
    }

    #[test]
    fn mixed_format() {
        let input = "[et_pb_section]Hello<br>World[/et_pb_section]";
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), 5);
        assert_eq!(output[0].name, "et_pb_section");
        assert_eq!(output[0].tag_type, tokenizer::TagType::Open);
        assert_eq!(output[0].attributes, None);
        assert_eq!(output[0].content, None);
        assert_eq!(output[0].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[1].name, "text");
        assert_eq!(output[1].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[1].attributes, None);
        assert_ne!(output[1].content, None);
        assert_eq!(output[1].content, Some(String::from("Hello")));

        assert_eq!(output[2].name, "br");
        assert_eq!(output[2].tag_type, tokenizer::TagType::Void);
        assert_eq!(output[2].attributes, None);
        assert_eq!(output[2].content, None);
        assert_eq!(output[2].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[3].name, "text");
        assert_eq!(output[3].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[3].attributes, None);
        assert_ne!(output[3].content, None);
        assert_eq!(output[3].content, Some(String::from("Hello")));

        assert_eq!(output[4].name, "et_pb_section");
        assert_eq!(output[4].tag_type, tokenizer::TagType::Close);
        assert_eq!(output[4].attributes, None);
        assert_eq!(output[4].content, None);
        assert_eq!(output[4].token_type, tokenizer::TokenType::Tag);
    }

    #[test]
    fn unmatched_tags() {
        let input = "[et_pb_section]Two<One[/et_pb_section]";
        let output = tokenizer::tokenize(input);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0].name, "et_pb_section");
        assert_eq!(output[0].tag_type, tokenizer::TagType::Open);
        assert_eq!(output[0].attributes, None);
        assert_eq!(output[0].content, None);
        assert_eq!(output[0].token_type, tokenizer::TokenType::Tag);

        assert_eq!(output[1].name, "text");
        assert_eq!(output[1].token_type, tokenizer::TokenType::Text);
        assert_eq!(output[1].attributes, None);
        assert_ne!(output[1].content, None);
        assert_eq!(output[1].content, Some(String::from("Two<One")));

        assert_eq!(output[2].name, "et_pb_section");
        assert_eq!(output[2].tag_type, tokenizer::TagType::Close);
        assert_eq!(output[2].attributes, None);
        assert_eq!(output[2].content, None);
        assert_eq!(output[2].token_type, tokenizer::TokenType::Tag);
    }
}
