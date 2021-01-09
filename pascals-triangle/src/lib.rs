pub struct PascalsTriangle {
    pub count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = vec![];
        for r in 0..self.count {
            let mut row = vec![];

            for c in 0..r + 1 {
                if c == 0 || c == r {
                    row.push(1);
                } else {
                    let elt =
                        rows[r as usize - 1][c as usize - 1] + rows[r as usize - 1][c as usize];
                    row.push(elt);
                }
            }
            rows.push(row);
        }
        rows
    }
}
