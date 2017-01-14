fn value(c: char) -> u32 {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    }
}

pub fn score(word: &str) -> u32 {
    let owned_word = String::from(word).to_lowercase();

    owned_word.chars().fold(0, |acc, x| acc + value(x))
}