use poker;

fn main() {
    let t1 = poker::winning_hands(&["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"]);
    println!();
    assert_eq!(t1, Some(["3S 4S 5D 6H JH"].to_vec()));
}
