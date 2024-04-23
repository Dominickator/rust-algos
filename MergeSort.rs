fn merge(arr1: &Vec<i32>, arr2: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut result: Vec<i32> = Vec::new();
    while i < arr1.len() && j < arr2.len() {
        if arr2[j] > arr1[i] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    return result;
}

fn merge_sort(arr: &Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.clone();
    }
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[0..mid].to_vec());
    let right = merge_sort(&arr[mid..].to_vec());
    merge(&left, &right)
}

fn main() {
    // Test merge function
    let arr1 = vec![1, 3, 5];
    let arr2 = vec![2, 4, 6];
    let merged_arr = merge(&arr1, &arr2);
    assert_eq!(merged_arr, [1, 2, 3, 4, 5, 6]);

    // Test merge_sort function
    let arr = vec![5, 2, 6, 1, 3, 4];
    let sorted_arr = merge_sort(&arr);
    assert_eq!(sorted_arr, [1, 2, 3, 4, 5, 6]);

    // Additional tests for code coverage
    let empty_arr: Vec<i32> = vec![];
    let empty_sorted_arr = merge_sort(&empty_arr);
    assert_eq!(empty_sorted_arr, []);

    let single_element_arr = vec![5];
    let single_element_sorted_arr = merge_sort(&single_element_arr);
    assert_eq!(single_element_sorted_arr, [5]);

    let sorted_arr = vec![1, 2, 3, 4, 5];
    let sorted_sorted_arr = merge_sort(&sorted_arr);
    assert_eq!(sorted_sorted_arr, [1, 2, 3, 4, 5]);

    let reverse_sorted_arr = vec![5, 4, 3, 2, 1];
    let reverse_sorted_sorted_arr = merge_sort(&reverse_sorted_arr);
    assert_eq!(reverse_sorted_sorted_arr, [1, 2, 3, 4, 5]);
}
