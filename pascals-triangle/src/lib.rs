pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => Vec::new(),
            _ => (1..self.row_count).fold(vec![vec![1]], |res, _| append_next_row(res)),
        }
    }
}

fn append_next_row(mut rows: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let end_row = rows.last().unwrap().to_vec();
    let end_row_iter_left = end_row[0..end_row.len() - 1].iter();
    let end_row_iter_right = end_row[1..end_row.len()].iter();

    let mut next_row: Vec<u32> = end_row_iter_left.zip(end_row_iter_right)
        .fold(vec![1], |mut res, (x, y)| {
            res.push(x + y);
            res
        });
    next_row.push(1);

    rows.push(next_row);
    rows
}
