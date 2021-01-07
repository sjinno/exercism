use poker;

fn main() {
    // let t1 = poker::winning_hands(&["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"]);
    let t2 = poker::winning_hands(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"]);
    println!();
    // assert_eq!(t1, Some(["3S 4S 5D 6H JH"].to_vec()));
    assert_eq!(t2, Some(["7S 8S 9S 6S 10S"].to_vec()));
}
