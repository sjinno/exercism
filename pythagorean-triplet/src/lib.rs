use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut hs = HashSet::new();
    for a in 1..sum / 3 {
        for b in a..sum / 2 {
            let c = sum - a - b;
            if b < c && a.pow(2) + b.pow(2) == c.pow(2) {
                hs.insert([a, b, c]);
            }
        }
    }
    hs
}
