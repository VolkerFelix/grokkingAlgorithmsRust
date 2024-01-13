use rand::prelude::*;

pub fn quick_sort(f_array: &[i32]) {
    if f_array.len() < 2 {
        return ;
    }
    else {
        let pivot = rand::thread_rng().gen_range(0..f_array.len()-1);
        let mut smaller = Vec::new();
        let mut bigger = Vec::new();
        for element in f_array {
            if f_array[pivot] < *element {
                bigger.push(element.clone());
            }
            if f_array[pivot] >= *element {
                smaller.push(element.clone());
            }
        }
        quick_sort(&smaller);
        quick_sort(&bigger);
    }
}