pub fn is_valid(num: &str) -> bool {
    // Strip out whitespace and reverse the iterator so we can get
    // 'every other digit starting from the right'
    let rev_chars = num.chars().filter(|x| !x.is_whitespace()).rev();

    // I use a for instead of more functional approach so we can fail early
    // and only iterate through the characters once at most. I can't think
    // of a functional approach that doesn't require multiple iterations.
    let mut sum = 0;
    for (i, x) in rev_chars.enumerate() {
        if let Some(n) = x.to_digit(10) {
            sum += double_mod_9_every_other(i, n);
        } else {
            return false;
        }
    }

    // The sum > 0 check is for the special case of a single '0' after
    // whitespace stripping. Any other length-1 string will fail the % check
    // anyway.
    sum > 0 && sum % 10 == 0
}

fn double_mod_9_every_other(i: usize, val: u32) -> u32 {
    match (i + 1) % 2 {
        0 => (2 * val) % 9,
        _ => val,
    }
}
