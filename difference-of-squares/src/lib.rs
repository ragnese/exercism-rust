pub fn sum_of_squares(n: i64) -> i64 {
    (1..n+1).map(|x| x*x).sum()
}

pub fn square_of_sum(n: i64) -> i64 {
    let sum: i64 = (1..n+1).sum();
    sum * sum
}

pub fn difference(n: i64) -> i64 {
    square_of_sum(n) - sum_of_squares(n)
}
