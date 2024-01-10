pub fn get_index_of_number(f_sorted_array: &[i32], f_number: i32) -> Option<usize> {
    let mut idx_first = 0;
    let mut idx_last = f_sorted_array.len() - 1;
    let mut middle_element = 0;

    while idx_first <= idx_last {
        middle_element = (idx_first + idx_last) / 2;

        if f_sorted_array[middle_element] == f_number  {
            return Some(middle_element);
        }
        else if f_sorted_array[middle_element] < f_number {
            idx_first = middle_element + 1;
        }
        else {
            idx_last = middle_element - 1;
        }
    }
    return None;
}