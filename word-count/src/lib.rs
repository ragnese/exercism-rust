use std::collections::HashMap;

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence.to_lowercase()
        // Split into "words"
        .split_whitespace()
        // Remove punctuation from each word
        .map(|word| {
            word.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        // Get rid of now-empty words
        .filter(|word| !word.is_empty())
        // For each word iterate its value in the map, or enter it with a 1
        .fold(HashMap::new(), |mut result, word| {
            *result.entry(word).or_insert(0) += 1;
            result
        })
}
