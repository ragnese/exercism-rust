pub fn primes_up_to(limit: u32) -> Vec<u32> {
    // Initialize vector with all possible values up to limit.
    let result: Vec<u32> = (2..limit + 1).collect();
    // Let's do some recursion.
    remove_multiples(0, result)
}

fn remove_multiples(idx: usize, list: Vec<u32>) -> Vec<u32> {
    // If the index of the next prime is outside the list, we're done.
    if idx >= list.len() {
        return list;
    }
    // Get the prime for the next pass of the sieve.
    let p = *list.get(idx).unwrap();

    // Remove elements that are divisible by p (but not p, itself).
    let next: Vec<u32> = list.into_iter().filter(|&x| x % p != 0 || x == p).collect();
    // Do the next pass.
    remove_multiples(idx + 1, next)
}
