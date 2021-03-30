pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec![String::from("A")];
    }

    // Count from B to some alphabet `c`.
    let len = ('B'..=c).count();

    //# Initial state:
    //
    // MAX_WIDTH & HEIGHT = 2 * len - 1
    let mut diamond = vec![vec![' '; 2 * len + 1]; 2 * len + 1];
    // Very top.
    diamond[0][len] = 'A';
    // Very bottom.
    diamond[2 * len][len] = 'A';
    // Center.
    diamond[len][0] = c;
    diamond[len][2 * len] = c;
    //# Initial state ends.

    for (i, c) in ('B'..c).enumerate() {
        let tsr = i + 1; // Top side row.
        let bsr = 2 * len - (i + 1); // Bottom side row.
        let lsc = len - (i + 1); // Left side column.
        let rsc = len + (i + 1); // Right side column.
        diamond[tsr][lsc] = c;
        diamond[tsr][rsc] = c;
        diamond[bsr][lsc] = c;
        diamond[bsr][rsc] = c;
    }

    diamond
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect()
}
