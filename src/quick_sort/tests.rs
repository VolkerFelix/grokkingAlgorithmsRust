#[cfg(test)]

use crate::quick_sort::quick_sort::quick_sort;
use rand::random;

#[test]
fn test_quick_sort() {
    let vec_length = 10;
    let mut random_vec = Vec::new();
    for _i in 0..vec_length {
        random_vec.push(rand::random::<i32>());
    }

    let mut for_comparison = random_vec.clone();
    for_comparison.sort();

    quick_sort(&mut random_vec);

    assert_eq!(for_comparison, random_vec);
}