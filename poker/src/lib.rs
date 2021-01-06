/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let v: Vec<Vec<&str>> = vec![hands.to_vec()];
    let number_of_hands = hands.len();

    Some(v[0])
}
