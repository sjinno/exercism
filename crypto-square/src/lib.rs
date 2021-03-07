pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<_>>();

    let length = normalized.len();
    let rows = (length as f64).sqrt().floor() as usize;
    let cols;
    if rows * rows == length {
        cols = rows;
    } else {
        cols = rows + 1;
    }

    if rows != cols {
        for _ in 0..(rows * cols - length) {
            normalized.push(' ');
        }
    }

    let chunks = normalized.chunks(cols).collect::<Vec<_>>();
    let mut encrypted = String::new();
    for c in 0..cols {
        for r in 0..rows {
            encrypted.push_str(&chunks[r][c].to_string());
        }
        if c != cols - 1 {
            encrypted.push_str(" ");
        }
    }

    encrypted
}
