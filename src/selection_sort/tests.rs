#[cfg(test)]
use crate::selection_sort::selection_sort::selection_sort;

#[test]
fn test_selection_sort() {
    let unsorted = [1, 4, 3, 2, 7, 10, 6];
    let sort_ref = [1, 2, 3, 4, 6, 7, 10];

    let sorted = selection_sort(&unsorted);

    assert_eq!(sort_ref.to_vec(), sorted);
}