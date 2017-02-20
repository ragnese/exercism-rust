use std::ascii::AsciiExt;

pub fn rotate(text: &str, key: u8) -> String {
    text.chars()
        .map(|x| {
            // Only rotate ASCII letters
            if x.is_ascii() && x.is_alphabetic() {
                // To do modular arithmetic, set offset based on which
                // alphabet we're using: A-Z or a-z
                let offset = if x.is_uppercase() { b'A' } else { b'a' };
                // d is 0-26 for a-z or A-Z
                let d = x as u8 - offset;
                // Rotate with wrapping and convert back
                ((d + key) % 26 + offset) as char
            } else {
                x
            }
        })
        .collect()
}
