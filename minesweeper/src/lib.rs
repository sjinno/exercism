// use std::iter::FromIterator;

// pub fn annotate(minefield: &[&str]) -> Vec<String> {
//     // Edge cases.
//     // Get the length of rows.
//     let row_len = &minefield.len();
//     if *row_len == 0 {
//         return vec![];
//     }
//     // Get the length of columns.
//     let col_len = &minefield[0].len();
//     if *col_len == 0 {
//         return vec!["".to_string()];
//     }
//     // Edge cases end.

//     // Initialize cells:
//     // If row = 3 and col = 2, then this will produce
//     // v = [
//     //         [0, 0],
//     //         [0, 0],
//     //         [0, 0]
//     //     ]
//     let col_zeros = std::iter::repeat(0).take(*col_len);
//     let cols = Vec::from_iter(col_zeros);
//     let row_zeros = std::iter::repeat(cols).take(*row_len);
//     let mut v = Vec::from_iter(row_zeros);

//     // Replace mines(*) with 10:
//     // If the case is ["*.", "..", "**"], then v becomes
//     // [
//     //     [10,  0],
//     //     [ 0,  0],
//     //     [10, 10]
//     // ]
//     minefield.iter().enumerate().for_each(|(row, x)| {
//         x.chars().into_iter().enumerate().for_each(|(col, y)| {
//             if y == '*' {
//                 v[row][col] = 10;
//             }
//         })
//     });

//     // Find how many mines there are around '.'.
//     // Add 1 to every surrounding cell if found.
//     // For example, using the case above, v will become
//     // [                  [               [
//     //     [10,  1],        [10,  1],        [10,  1],
//     //     [ 1,  1],  =>    [ 2,  2],  =>    [ 3,  3],
//     //     [10, 10]         [10, 11]         [11, 11]
//     // ]                  ]               ]
//     minefield.iter().enumerate().for_each(|(row, mf)| {
//         mf.chars()
//             .into_iter()
//             .enumerate()
//             .for_each(|(col, cell)| match cell {
//                 '*' => get_mines(&mut v, row, col, *row_len, *col_len),
//                 _ => {}
//             })
//     });

//     // Finally convert every row in v into String and append to vec![].
//     // Note: Any cell with more than 10 will be turned into "*".
//     let mut ans = vec![];
//     v.iter().for_each(|row| {
//         let mut res = String::new();
//         row.iter().for_each(|cell| match cell {
//             0 => res.push_str(" "),
//             1..=8 => res.push_str(&cell.to_string()),
//             _ => res.push_str("*"),
//         });
//         ans.push(res);
//     });

//     ans
// }

// fn get_mines(v: &mut Vec<Vec<usize>>, row: usize, col: usize, row_len: usize, col_len: usize) {
//     // Left
//     if col > 0 {
//         update_cell(v, row, col - 1);
//     }
//     // Right
//     if col < col_len - 1 {
//         update_cell(v, row, col + 1);
//     }

//     // Above
//     if row > 0 {
//         update_cell(v, row - 1, col);
//     }
//     // Beneath
//     if row < row_len - 1 {
//         update_cell(v, row + 1, col);
//     }

//     // Upper left
//     if row > 0 && col > 0 {
//         update_cell(v, row - 1, col - 1);
//     }

//     // Upper right
//     if row > 0 && col < col_len - 1 {
//         update_cell(v, row - 1, col + 1);
//     }

//     // Lower left
//     if row < row_len - 1 && col > 0 {
//         update_cell(v, row + 1, col - 1);
//     }

//     // Lower right
//     if row < row_len - 1 && col < col_len - 1 {
//         update_cell(v, row + 1, col + 1);
//     }
// }

// fn update_cell(v: &mut Vec<Vec<usize>>, row: usize, col: usize) {
//     v[row][col] += 1;
// }

// // Community solution 1
// // How does this work??? LOL
// static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
//     (-1, -1),
//     (0, -1),
//     (1, -1),
//     (-1, 0),
//     (1, 0),
//     (-1, 1),
//     (0, 1),
//     (1, 1),
// ];

// pub fn annotate(field: &[&str]) -> Vec<String> {
//     let height = field.len() as i32;
//     (0..height)
//         .map(|y| {
//             let width = field[y as usize].len() as i32;
//             (0..width)
//                 .map(|x| {
//                     if field[y as usize].as_bytes()[x as usize] == b'*' {
//                         '*'
//                     } else {
//                         match NEIGBOURHOOD_OFFSETS
//                             .iter()
//                             .map(|&(ox, oy)| (x + ox, y + oy))
//                             .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
//                             .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
//                             .count()
//                         {
//                             0 => ' ',
//                             n => (n as u8 + '0' as u8) as char,
//                         }
//                     }
//                 })
//                 .collect()
//         })
//         .collect()
// }

// Community solution 2
// ???
#![deny(clippy::pedantic)]
use std::{char::from_digit, convert::TryFrom};

#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|y| {
            (0..minefield[y].len())
                .map(|x| match minefield[y].as_bytes()[x] {
                    b'*' => '*',
                    _ => match (x.saturating_sub(1)..=x + 1)
                        .flat_map(|x| (y.saturating_sub(1)..=y + 1).map(move |y| (x, y)))
                        .filter_map(|(x, y)| minefield.get(y)?.as_bytes().get(x))
                        .filter(|&&x| x == b'*')
                        .count()
                    {
                        0 => ' ',
                        x => from_digit(u32::try_from(x).unwrap(), 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect()
}
