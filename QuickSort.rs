fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low; // Start from 'low' instead of 'low - 1'

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i // Removed unnecessary casting
}

fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if high > low {
        // Changed to avoid underflow and check bounds correctly
        let pi = partition(arr, low, high);

        if pi > 0 {
            // Check if `pi` is greater than 0 to avoid underflow
            quick_sort(arr, low, pi - 1);
        }
        quick_sort(arr, pi + 1, high);
    }
}

fn main() {
    // Tests remain the same
    let mut arr = vec![];
    let mut n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![]);

    let mut arr = vec![1];
    n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![1]);

    let mut arr = vec![5, 3, 1, 2, 6, 4];
    n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

    let mut arr = vec![6, 5, 4, 3, 2, 1];
    n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

    let mut arr = vec![1, 2, 3, 4, 5, 6];
    n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

    let mut arr = vec![2, 2, 2, 2, 2, 2];
    n = arr.len().saturating_sub(1);
    quick_sort(&mut arr, 0, n);
    assert_eq!(arr, vec![2, 2, 2, 2, 2, 2]);
}
