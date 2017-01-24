use std::collections::HashSet;
use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_lowercase().chars() // lowercase makes A = a for line 7
        .filter(|x| x.is_alphabetic() && x.is_ascii()) // only take chars that are a-z
        .collect::<HashSet<char>>() // uniquify
        .len() == 26
}
