#![forbid(unsafe_code)]

pub fn where_k_th_ordinal_element_greater<'a>(
    lhs: &'a Vec<i32>,
    rhs: &'a Vec<i32>,
    k: usize,
) -> &'a Vec<i32> {
    let mut lhs_sorted = lhs.clone();
    let mut rhs_sorted = rhs.clone();
    lhs_sorted.sort();
    rhs_sorted.sort();

    if lhs_sorted[k] > rhs_sorted[k] {
        lhs
    } else {
        rhs
    }
}
