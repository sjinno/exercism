#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (f, s) if f < s && is_sublist(first_list, second_list) => Comparison::Sublist,
        (f, s) if f == s && first_list == second_list => Comparison::Equal,
        (f, s) if f > s && is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    long_list.windows(short_list.len()).any(|w| w == short_list)
}
