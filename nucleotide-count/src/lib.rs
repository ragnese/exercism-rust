use std::collections::HashMap;
use std::collections::hash_map::Entry;

const LIBRARY: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(c: char, seq: &str) -> Result<usize, String> {
    if LIBRARY.contains(&c) && seq.chars().all(|x| LIBRARY.contains(&x)) {
        Ok(seq.chars().filter(|x| x == &c).count())
    } else {
        Err("Not a nucleotide!".to_owned())
    }
}

pub fn nucleotide_counts(seq: &str) -> Result<HashMap<char, usize>, String> {
    let mut result = HashMap::new();
    result.insert(LIBRARY[0], 0);
    result.insert(LIBRARY[1], 0);
    result.insert(LIBRARY[2], 0);
    result.insert(LIBRARY[3], 0);

    for c in seq.chars() {
        match result.entry(c) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            _ => return Err("Non-nucleotide in sequence!".to_owned()),
        }
    };

    Ok(result)
}
