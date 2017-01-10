pub fn hamming_distance(seq1: &str, seq2: &str) -> Result<usize, String> {
    if seq1.len() == seq2.len() {
        Ok(seq1.chars()
            .zip(seq2.chars()) // zip the two strings together into iter of pairs
            .filter(|&(a, b)| a != b) // take only ones where seq1[i] != seq2[i]
            .count())
    } else {
        Err("Error: Hamming distance is only defined for sequences of equal length.".to_string())
    }
}
