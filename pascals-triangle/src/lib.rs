// pub struct PascalsTriangle {
//     pub count: u32,
// }

// impl PascalsTriangle {
//     pub fn new(row_count: u32) -> Self {
//         PascalsTriangle { count: row_count }
//     }

//     pub fn rows(&self) -> Vec<Vec<u32>> {
//         let mut rows: Vec<Vec<u32>> = vec![];
//         for r in 0..self.count {
//             let mut row = vec![];

//             for c in 0..r + 1 {
//                 if c == 0 || c == r {
//                     row.push(1);
//                 } else {
//                     let elt =
//                         rows[r as usize - 1][c as usize - 1] + rows[r as usize - 1][c as usize];
//                     row.push(elt);
//                 }
//             }
//             rows.push(row);
//         }
//         rows
//     }
// }

// Community solution 1
use std::iter;

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        match self.row_count {
            0 => Vec::new(),
            1 => vec![vec![1]],
            r => {
                let mut rows = Self::new(r - 1).rows();
                let new_row = {
                    let last_row = rows.last().unwrap();
                    let zip1 = iter::once(0).chain(last_row.iter().cloned());
                    let zip2 = last_row.iter().cloned().chain(iter::once(0));
                    zip1.zip(zip2).map(|(n, m)| n + m).collect()
                };
                rows.push(new_row);
                rows
            }
        }
    }
}
