pub fn get_index_of_number(f_sorted_array: &[i32], f_number: i32) -> Option<usize> {
    let mut idx_first = 0;
    let mut idx_last = f_sorted_array.len() - 1;
    let mut middle_element = (idx_first + idx_last) / 2;

    while idx_first != idx_last {
        if f_sorted_array[middle_element] == f_number  {
            return Some(middle_element);
        }
        else if f_sorted_array[middle_element] < f_number {
            idx_first = middle_element + 1;
            println!("First idx: {}", idx_first);
        }
        else {
            idx_last = middle_element - 1;
            println!("Last idx: {}", idx_last);
        }
        middle_element = (idx_first + idx_last) / 2;
        println!("Middle idx: {}", middle_element);

    }
    if f_sorted_array[middle_element] == f_number  {
        return Some(middle_element);
    }
    return None;
}