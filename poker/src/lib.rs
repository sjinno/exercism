/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let v_hands = hands.to_vec();
    let num_of_hands = v_hands.len();
    if num_of_hands == 1 {
        return Some(v_hands);
    }

    for hand in v_hands {
        println!("{}", hand);
    }

    None
}
