#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_super_list<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    for start_idx in 0..first_list.len() {
        let slice = &first_list[start_idx..];
        if slice.starts_with(&second_list) {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // Equal check: both lists are entirely equal
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    // Sublist check
    if is_super_list(_first_list, _second_list) {
        return Comparison::Superlist;
    }

    // Superlist check
    if is_super_list(_second_list, _first_list) {
        return Comparison::Sublist;
    }

    // Unequal check: if its not any of the above,
    // then it has to be unequal
    Comparison::Unequal
}
