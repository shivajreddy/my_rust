#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    // Equal check: both lists are entirely equal
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    // Sublist check:

    // Superlist check:

    // Unequal check: if its not any of the above,
    // then it has to be unequal
    Comparison::Unequal
}
