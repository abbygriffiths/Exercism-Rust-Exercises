#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    for (x, y) in first_list.iter().zip(second_list) {
        if x != y {
            return Comparison::Unequal;
        }
    }

    Comparison::Unequal
}
