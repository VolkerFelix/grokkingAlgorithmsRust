#[cfg(test)]
use crate::binary_search::binary_search::get_index_of_number;

#[test]
fn test_binary_search_number_found() {
    let sorted_array = [ 1, 2, 5, 7, 13, 15];
    let looking_for_number= 15;

    let index = get_index_of_number(&sorted_array, looking_for_number);

    assert_eq!(5, index.unwrap());
}

#[test]
fn test_binary_search_number_not_found() {
    let sorted_array = [ 1, 2, 5, 7, 13, 15];
    let looking_for_number= 16;

    let index = get_index_of_number(&sorted_array, looking_for_number);

    assert_eq!(None, index);
}