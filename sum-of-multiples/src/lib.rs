use std::collections::HashSet;

pub fn sum_of_multiples<'a>(n: i64, ms: &'a [i64]) -> i64 {
    let mut set = HashSet::new();

    for &m in ms.iter() {
        let mut mult: i64 = m;
        while mult < n {
            set.insert(mult);
            mult += m;
        }
    }

    set.iter().sum()
}
