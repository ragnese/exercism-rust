pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // This is the "elegant" and fancy functional answer
    // (1..64+1).map(|x| square(x)).sum()

    // The answer is 2^64 - 1, but 2^64 is one larger than the largest
    // possible u64, so we have to do trickery
    (2u64.pow(63) - 1) * 2 + 1
}
