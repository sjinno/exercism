use std::iter::FromIterator;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Edge cases.
    // Get the length of rows.
    let row_len = &minefield.len();
    if *row_len == 0 {
        return vec![];
    }
    // Get the length of columns.
    let col_len = &minefield[0].len();
    if *col_len == 0 {
        return vec!["".to_string()];
    }
    // Edge cases end.

    // Initialize cells:
    // If row = 3 and col = 2, then this will produce
    // v = [
    //         [0, 0],
    //         [0, 0],
    //         [0, 0]
    //     ]
    let col_zeros = std::iter::repeat(0).take(*col_len);
    let cols = Vec::from_iter(col_zeros);
    let row_zeros = std::iter::repeat(cols).take(*row_len);
    let mut v = Vec::from_iter(row_zeros);

    // Replace mines(*) with 10:
    // If the case is ["*.", "..", "**"], then v becomes
    // [
    //     [10,  0],
    //     [ 0,  0],
    //     [10, 10]
    // ]
    minefield.iter().enumerate().for_each(|(row, x)| {
        x.chars().into_iter().enumerate().for_each(|(col, y)| {
            if y == '*' {
                v[row][col] = 10;
            }
        })
    });

    // Find how many mines there are around '.'.
    // Add 1 to every surrounding cell if found.
    // For example, using the case above, v will become
    // [                  [               [
    //     [10,  1],        [10,  1],        [10,  1],
    //     [ 1,  1],  =>    [ 2,  2],  =>    [ 3,  3],
    //     [10, 10]         [10, 11]         [11, 11]
    // ]                  ]               ]
    minefield.iter().enumerate().for_each(|(row, mf)| {
        mf.chars()
            .into_iter()
            .enumerate()
            .for_each(|(col, cell)| match cell {
                '*' => get_mines(&mut v, row, col, *row_len, *col_len),
                _ => {}
            })
    });

    // Finally convert every row in v into String and append to vec![].
    // Note: Any cell with more than 10 will be turned into "*".
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

fn get_mines(v: &mut Vec<Vec<usize>>, row: usize, col: usize, row_len: usize, col_len: usize) {
    // Left
    if col > 0 {
        update_cell(v, row, col - 1);
    }
    // Right
    if col < col_len - 1 {
        update_cell(v, row, col + 1);
    }

    // Above
    if row > 0 {
        update_cell(v, row - 1, col);
    }
    // Beneath
    if row < row_len - 1 {
        update_cell(v, row + 1, col);
    }

    // Upper left
    if row > 0 && col > 0 {
        update_cell(v, row - 1, col - 1);
    }

    // Upper right
    if row > 0 && col < col_len - 1 {
        update_cell(v, row - 1, col + 1);
    }

    // Lower left
    if row < row_len - 1 && col > 0 {
        update_cell(v, row + 1, col - 1);
    }

    // Lower right
    if row < row_len - 1 && col < col_len - 1 {
        update_cell(v, row + 1, col + 1);
    }
}

fn update_cell(v: &mut Vec<Vec<usize>>, row: usize, col: usize) {
    v[row][col] += 1;
}
