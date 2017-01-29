pub fn lsp(input: &str, size: usize) -> Result<u32, ()> {
    if size == 0 { // Yay for special cases
        return Ok(1);
    }

    // Converts input into a vector of digits or return error if there is a 
    // non-digit in the input.
    let digits = input.chars()
        .map(|x| x.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .ok_or(())?;

    // Get all windows over the vector, take their products and return the max.
    digits.windows(size)
        .map(|x| x.iter().product())
        .max()
        .ok_or(())
}
