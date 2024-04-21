fn linear_search(arr: &Vec<i32>, goal: i32) -> Option<usize> {
    for (i, &num) in arr.iter().enumerate() {
        if num == goal {
            return Some(i);
        }
    }
    None
}

fn main() {
    let arr = vec![1, 34, 564, 34, 213];
    let target = 213;
    let result = linear_search(&arr, target);

    match result {
        Some(index) => println!("{} was found in the array at index {}!", target, index),
        None => println!("{} was not found in the array", target),
    }
}
