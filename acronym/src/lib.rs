/* This solution is AWFUL. I'm surprised this problem kicked my ass so badly.
 * There are far too many allocations in my solution and it's nearly 
 * unreadable
 */

pub fn abbreviate(name: &str) -> String {
    // Split into words by whitespace or the - character
    name.split(|x| (x as char) == '-' || (x as char).is_whitespace())
        // For each word, strip non-letters and then turn it into an acronmyn
        // piece- usually it's just the first letter, unless it's CamelCase.
        .map(|x| {
            // allocation
            let word = x.chars().filter(|c| c.is_alphabetic()).collect();
            acronymize_word(word)
        })
        // allocation
        .collect()
}

fn acronymize_word(word: String) -> String {
    let mut c_iter = word.chars();
    // allocation
    let mut result = c_iter.next().unwrap().to_uppercase().collect();
    // allocation
    let caps = c_iter.filter(|c| c.is_uppercase()).collect::<String>();
    if caps.len() == word.len() - 1 {
        result
    } else {
        // allocation
        result.push_str(&caps);
        result
    }
}
