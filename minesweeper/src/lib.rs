use std::iter::FromIterator;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_len = &minefield[0].len();
    let col_len = &minefield.len();
    let row_zeros = std::iter::repeat(0).take(*row_len);
    let rows = Vec::from_iter(row_zeros);
    let col_zeros = std::iter::repeat(rows).take(*col_len);
    let mut v = Vec::from_iter(col_zeros);
    // println!("{:?}", v);
    minefield.iter().enumerate().for_each(|(row, x)| {
        x.chars().into_iter().enumerate().for_each(|(col, y)| {
            if y == '*' {
                v[row][col] = 10;
            }
        })
    });
    println!("{:?}", v);

    minefield.iter().enumerate().for_each(|(row, mf)| {
        mf.chars()
            .into_iter()
            .enumerate()
            .for_each(|(col, x)| match x {
                '*' => {
                    // Upper left
                    if row > 0 && col > 0 {
                        if let Some(num) = v[row - 1].get_mut(col - 1) {
                            *num += 1;
                        }
                    }
                    if row > 0 {
                        // Above
                        if let Some(num) = v[row - 1].get_mut(col) {
                            *num += 1;
                        }
                        // Upper right
                        if let Some(num) = v[row - 1].get_mut(col + 1) {
                            *num += 1;
                        }
                    }
                    if col > 0 {
                        // Left
                        if let Some(num) = v[row].get_mut(col - 1) {
                            *num += 1;
                        }
                        if row < *row_len - 1 {
                            // Lower left
                            if let Some(num) = v[row + 1].get_mut(col - 1) {
                                *num += 1;
                            }
                        }
                    }
                    // Right
                    if let Some(num) = v[row].get_mut(col + 1) {
                        *num += 1;
                    }
                    // Lower right
                    if row < *row_len - 1 {
                        if let Some(num) = v[row + 1].get_mut(col + 1) {
                            *num += 1;
                        }
                        // Beneath
                        if let Some(num) = v[row + 1].get_mut(col) {
                            *num += 1;
                        }
                    }
                }
                _ => {}
            })
    });

    println!("{:?}", v);
    let mut ans = vec![];
    v.iter().for_each(|row| {
        let mut res = String::new();
        row.iter().for_each(|cell| match cell {
            0 => res.push_str(" "),
            1..=8 => res.push_str(&cell.to_string()),
            _ => res.push_str("*"),
        });
        ans.push(res);
    });

    ans
}

// use std::iter::FromIterator;

// fn main() {
//     // let s: &str = "*.*";
//     // let col_len = s.len();
//     // let zeros = std::iter::repeat(0).take(col_len);
//     // let mut v = Vec::from_iter(zeros);
//     // println!("{:?}", v);
//     // s.chars().into_iter()
//     //         .enumerate()
//     //         .for_each(|(i, x)| {
//     //           if x == '*' { v[i] = 10; }
//     //         });
//     // println!("{:?}", v);
//     let s: &[&str] = &["*.*", "*.*", "*.*"];
//     let row_len = s[0].len();
//     let col_len = s.len();
//     // println!("{}, {}", row_len, col_len);
//     let z = std::iter::repeat(0).take(row_len);
//     let vr = Vec::from_iter(z);
//     let zeros = std::iter::repeat(vr).take(col_len);
//     let mut v = Vec::from_iter(zeros);
//     println!("{:?}", v);
//     s.iter().enumerate().for_each(|(row, x)| {
//         x.chars().into_iter().enumerate().for_each(|(col, y)| {
//             if y == '*' {
//                 v[row][col] = 10;
//             }
//         })
//     });
//     // println!("{:?}", v);
// }

// #[derive(Debug)]
// enum IpAddress {
//     V4(u32, u32, u32, u32),
// }

// impl From<[u32; 4]> for IpAddress {
//     fn from(v4: [u32; 4]) -> Self {
//         IpAddress::V4(v4[0], v4[1], v4[2], v4[3])
//     }
// }

// fn main() {
//     let x: IpAddress = [127, 0, 0, 1].into();
//     println!("{:?}", x);
// }
