#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (l1, l2) => {
            if l1 > l2 {
                if is_sublist(_first_list, _second_list) {
                    Comparison::Superlist
                } else {
                    Comparison::Unequal
                }
            } else if l1 < l2 {
                if is_sublist(_second_list, _first_list) {
                    Comparison::Sublist
                } else {
                    Comparison::Unequal
                }
            } else {
                if _first_list == _second_list {
                    Comparison::Equal
                } else {
                    Comparison::Unequal
                }
            }
        }
    }
}

fn is_sublist<T: PartialEq>(f_list: &[T], s_list: &[T]) -> bool {
    s_list.is_empty() || f_list.windows(s_list.len()).any(|list| list == s_list)
}
