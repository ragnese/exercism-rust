use std::ascii::AsciiExt;

pub fn encode(text: &str) -> String {
    text.chars()
        // Remove all punctuation and non-ascii
        .filter(|x| x.is_alphanumeric() && x.is_ascii())
        // Convert to lowercase and encode with atbash cypher
        .map(|x| atbash(x.to_ascii_lowercase()))
        .enumerate()
        // Build a string while inserting a space after every fifth character
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 5 == 0 {
                acc.push(' ');
            }
            acc.push(c);
            acc
        })
}

pub fn decode(text: &str) -> String {
    text.chars()
        // Remove all punctuation and non-ascii
        .filter(|x| x.is_alphanumeric() && x.is_ascii())
        // Convert to lowercase and encode with atbash cypher
        .map(|x| atbash(x.to_ascii_lowercase()))
        // Collect into a string
        .collect()
}

/* This function is ugly. I should use the ASCII codes and use arithmetic
 * like x -> (x + (25 - 2 * (x - 13))) % 26 where x is ASCII# - 97,
 * but I'm too lazy to figure out how to work between char and ASCII.
 */
fn atbash(c: char) -> char {
    match c {
        'a' => 'z',
        'b' => 'y',
        'c' => 'x',
        'd' => 'w',
        'e' => 'v',
        'f' => 'u',
        'g' => 't',
        'h' => 's',
        'i' => 'r',
        'j' => 'q',
        'k' => 'p',
        'l' => 'o',
        'm' => 'n',
        'n' => 'm',
        'o' => 'l',
        'p' => 'k',
        'q' => 'j',
        'r' => 'i',
        's' => 'h',
        't' => 'g',
        'u' => 'f',
        'v' => 'e',
        'w' => 'd',
        'x' => 'c',
        'y' => 'b',
        'z' => 'a',
        _ => c,
    }
}
