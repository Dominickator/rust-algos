fn insertion_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut i = 1;
    while i < arr.len() {
        let x = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > x {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = x;
        i += 1;
    }
    return arr.to_vec();
}

fn main() {
    let mut test = vec![1, 5, 3, 7, 4, 8, 9, 34, 12, 56, 100];
    insertion_sort(&mut test);
    for i in 0..test.len() - 1 {
        if i + 1 >= test.len() {
            assert!(test[i + 1] >= test[i], "TEST FAILED!");
        }
    }
    println!("{:?}", test);
}
