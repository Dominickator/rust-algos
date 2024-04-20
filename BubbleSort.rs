fn bubble_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut n = arr.len();
    let mut swapped: bool = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                let temp = arr[i - 1];
                arr[i - 1] = arr[i];
                arr[i] = temp;
                swapped = true;
            }
        }
        n -= 1;
    }
    return arr.to_vec();
}

fn main() {
    let mut test1 = vec![1, 3, 7, 4, 0];
    test1 = bubble_sort(&mut test1);
    println!("{:?}", test1);
}
