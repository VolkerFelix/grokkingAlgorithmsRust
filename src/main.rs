mod binary_search;

//use crate::binary_search::binary_search;

fn main() {

    println!("Hello, world!");
}

#[cfg(test)]
#[test]

fn test_binary_search() {
    let sorted_array = [ 1, 2, 5, 7, 13, 15];
    let looking_for_number= 5;

    let index = binary_search::binary_search::get_index_of_number(&sorted_array, looking_for_number);

    assert_eq!(2, index.unwrap());
}