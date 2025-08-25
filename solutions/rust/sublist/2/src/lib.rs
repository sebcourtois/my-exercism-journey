#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn _is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.is_empty() { return true; }

    for window in _second_list.windows(_first_list.len()) {
        if window == _first_list { return true; }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let first_len = _first_list.len();
    let second_len = _second_list.len();

    if first_len > second_len {
        if _is_sublist(_second_list, _first_list) {
            return Comparison::Superlist;
        }
    } else {
        if _is_sublist(_first_list, _second_list) {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}
