pub fn selection_sort(f_unsorted_array: &[i32]) -> Vec<i32> {
    let mut unsorted: Vec<i32> = f_unsorted_array.to_vec();
    let mut sorted = Vec::new();

    for _element_idx in 0..unsorted.len() {
        let (idx, item) = find_smallest_element(&unsorted);
        sorted.push(item);
        unsorted.remove(idx);
    }

    sorted
}

fn find_smallest_element(f_unsorted_array: &[i32]) -> (usize, i32) {
    let mut smallest_element = f_unsorted_array[0];
    let mut smallest_element_idx = 0;

    for (idx, element) in f_unsorted_array.iter().enumerate() {
        if element < &smallest_element {
            smallest_element_idx = idx;
            smallest_element = *element;
        }
    }
    (smallest_element_idx, smallest_element)
}