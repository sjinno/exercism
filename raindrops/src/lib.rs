pub fn raindrops(n: usize) -> String {
    let is_factor = |factor| n % factor == 0;
    let mut s = String::new();
    if is_factor(3) { s.push_str("Pling"); }
    if is_factor(5) { s.push_str("Plang"); }
    if is_factor(7) { s.push_str("Plong"); }
    if s.is_empty() { return n.to_string(); }
    s
}
