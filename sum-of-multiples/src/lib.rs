use std::collections::HashSet;

pub fn sum_of_multiples<'a>(n: i64, ms: &'a [i64]) -> i64 {
    ms.iter()
        .flat_map(|&base| {
            get_multiples(n, base).into_iter() // get multiples of base < n
        }) 
        .collect::<HashSet<i64>>() // HashSet doesn't store redundant entries
        .iter().sum() // Add them all up
}

fn get_multiples(n: i64, base: i64) -> Vec<i64> {
    (1..n / base + 1).map(|x| base * x).filter(|&x| x < n).collect()
}
