pub fn is_leap_year(year: u32) -> bool {
    // Year is divisible by 4 and either not divisible by 100 or divisible by
    // 400
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
