pub fn annotate(minefield: &[&str]) -> Vec<String> {
    s.iter().enumerate().for_each(|(i, x)| {
        x.chars().into_iter().enumerate().for_each(|(j, y)| {
            if y == '*' {
                v[i][j] = 10;
            }
        })
    });
    vec![]
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
