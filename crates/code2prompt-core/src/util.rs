//! This module contains util functions

/// Removes a UTF‑8 Byte Order Mark (BOM) from the beginning of a byte slice if present.
///
/// The UTF‑8 BOM is the byte sequence `[0xEF, 0xBB, 0xBF]`. This function checks whether
/// the provided slice starts with these bytes and, if so, returns a subslice without them.
/// Otherwise, it returns the original slice.
pub fn strip_utf8_bom(data: &[u8]) -> &[u8] {
    const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];
    if data.starts_with(BOM) {
        &data[BOM.len()..]
    } else {
        data
    }
}

/// Escapes special characters in text for metaprompt format.
///
/// This function replaces the following characters with their escape sequences:
/// - `<` with `[LESS_THAN]`
/// - `>` with `[GREATER_THAN]`
/// - `'` with `[SINGLE_QUOTE]`
/// - `"` with `[DOUBLE_QUOTE]`
/// - `` ` `` with `[BACKTICK]`
/// - `{` with `[OPEN_BRACE]`
/// - `}` with `[CLOSE_BRACE]`
/// - `[` with `[OPEN_BRACKET]`
/// - `]` with `[CLOSE_BRACKET]`
/// - `(` with `[OPEN_PAREN]`
/// - `)` with `[CLOSE_PAREN]`
///
/// # Arguments
///
/// * `text` - The input text to escape
///
/// # Returns
///
/// * `String` - The escaped text
pub fn escape_metaprompt_characters(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2); // Pre-allocate with some extra space
    
    for ch in text.chars() {
        match ch {
            '<' => result.push_str("[LESS_THAN]"),
            '>' => result.push_str("[GREATER_THAN]"),
            '\'' => result.push_str("[SINGLE_QUOTE]"),
            '"' => result.push_str("[DOUBLE_QUOTE]"),
            '`' => result.push_str("[BACKTICK]"),
            '{' => result.push_str("[OPEN_BRACE]"),
            '}' => result.push_str("[CLOSE_BRACE]"),
            '[' => result.push_str("[OPEN_BRACKET]"),
            ']' => result.push_str("[CLOSE_BRACKET]"),
            '(' => result.push_str("[OPEN_PAREN]"),
            ')' => result.push_str("[CLOSE_PAREN]"),
            _ => result.push(ch),
        }
    }
    
    result
}
