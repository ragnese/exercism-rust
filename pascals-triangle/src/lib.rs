pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(|x| calc_row(x)).collect()
    }
}

fn calc_row(n: u32) -> Vec<u32> {
    let mut next_row: Vec<u32> = Vec::with_capacity(n as usize);
    (1..n + 2).fold(1, |acc, x| {
        next_row.push(acc);
        acc * (n + 1 - x) / x
    });
    next_row
}
