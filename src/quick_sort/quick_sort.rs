/// Recursive implementation of Quick Sort.
/// Partition index is obtained and then
/// the function is called recursively with the elements before and after the pivot.
pub fn quick_sort(f_array: &mut[i32]) {
    if !f_array.is_empty() {
        let partition_idx = partition(f_array);
        let length = f_array.len();
        quick_sort(&mut f_array[0..partition_idx]);
        quick_sort(&mut f_array[partition_idx+1..length]);
    }
} 

/// Array will be modified in a way that:
/// Pivot is at its correct position as in a sorted array;
/// All elements before the pivot are less than the pivot but not necessarily sorted;
/// All elements after the pivot are greater but not necessarily sorted;
/// The index of the pivot is returned.
fn partition(f_array: &mut[i32]) -> usize {
    let pivot_idx = f_array.len()-1;
    let pivot = f_array[pivot_idx];
    let mut i = 0;
    let mut j = 0;

    while j < f_array.len()-1 {
        if f_array[j] <= pivot {
            f_array.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    f_array.swap(i, pivot_idx);
    i
}