use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut hs = HashSet::new();
    for a in 2..sum / 3 {
        for b in a + 1..sum / 2 {
            let c = sum - a - b;
            if b < c && a * a + b * b == c * c {
                hs.insert([a, b, c]);
            }
        }
    }
    hs
}
