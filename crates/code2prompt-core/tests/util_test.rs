use code2prompt_core::util::{strip_utf8_bom, escape_metaprompt_characters};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_utf8_bom_when_present() {
        let input = b"\xEF\xBB\xBFHello, world!";
        let expected = b"Hello, world!";
        let output = strip_utf8_bom(input);
        assert_eq!(
            output, expected,
            "BOM should be stripped from the beginning of the input."
        );
    }

    #[test]
    fn test_strip_utf8_bom_when_not_present() {
        let input = b"Hello, world!";
        let output = strip_utf8_bom(input);
        assert_eq!(
            output, input,
            "Input without a BOM should remain unchanged."
        );
    }

    #[test]
    fn test_strip_utf8_bom_empty_input() {
        let input = b"";
        let output = strip_utf8_bom(input);
        assert_eq!(
            output, input,
            "An empty input should return an empty output."
        );
    }

    #[test]
    fn test_strip_utf8_bom_only_bom() {
        let input = b"\xEF\xBB\xBF";
        let expected = b"";
        let output = strip_utf8_bom(input);
        assert_eq!(
            output, expected,
            "Input that is only a BOM should return an empty slice."
        );
    }

    #[test]
    fn test_escape_metaprompt_characters_basic() {
        let input = "Hello <world>";
        let expected = "Hello [LESS_THAN]world[GREATER_THAN]";
        let output = escape_metaprompt_characters(input);
        assert_eq!(output, expected, "Should escape angle brackets");
    }

    #[test]
    fn test_escape_metaprompt_characters_all_special() {
        let input = r#"<>"'`{}[]() test"#;
        let expected = "[LESS_THAN][GREATER_THAN][DOUBLE_QUOTE][SINGLE_QUOTE][BACKTICK][OPEN_BRACE][CLOSE_BRACE][OPEN_BRACKET][CLOSE_BRACKET][OPEN_PAREN][CLOSE_PAREN] test";
        let output = escape_metaprompt_characters(input);
        assert_eq!(output, expected, "Should escape all special characters");
    }

    #[test]
    fn test_escape_metaprompt_characters_no_double_escaping() {
        let input = "console.log(`Hello {world} [test] (example)`);";
        let output = escape_metaprompt_characters(input);
        // Verify that [OPEN_BRACKET] doesn't get double-escaped to [OPEN_BRACKET[OPEN_BRACKET]...
        assert!(!output.contains("[OPEN_BRACKET][OPEN_BRACKET]"));
        assert!(!output.contains("[CLOSE_BRACKET][CLOSE_BRACKET]"));
        
        let expected = "console.log[OPEN_PAREN][BACKTICK]Hello [OPEN_BRACE]world[CLOSE_BRACE] [OPEN_BRACKET]test[CLOSE_BRACKET] [OPEN_PAREN]example[CLOSE_PAREN][BACKTICK][CLOSE_PAREN];";
        assert_eq!(output, expected, "Should escape without double-escaping");
    }

    #[test]
    fn test_escape_metaprompt_characters_empty() {
        let input = "";
        let expected = "";
        let output = escape_metaprompt_characters(input);
        assert_eq!(output, expected, "Empty input should return empty output");
    }

    #[test]
    fn test_escape_metaprompt_characters_no_special() {
        let input = "Hello world without special chars";
        let expected = "Hello world without special chars";
        let output = escape_metaprompt_characters(input);
        assert_eq!(output, expected, "Text without special chars should remain unchanged");
    }
}
