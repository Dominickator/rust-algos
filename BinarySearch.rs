fn binary_search(arr: &Vec<i32>, goal: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut l = 0;
    let mut r = arr.len() - 1;
    while l <= r {
        let m = (l + r) / 2;

        if arr[m] == goal {
            return Some(m);
        }

        if arr[m] < goal {
            l = m + 1;
        } else {
            r = m - 1
        }
    }
    None
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5, 6];
    let target1 = 4;
    let result1 = binary_search(&arr1, target1);
    println!("Test 1 result: {}", result1 == Some(3));

    let arr2 = vec![1, 2, 3, 4, 5, 6];
    let target2 = 7;
    let result2 = binary_search(&arr2, target2);
    println!("Test 2 result: {}", result2 == None);

    let arr3: Vec<i32> = vec![];
    let target3 = 5;
    let result3 = binary_search(&arr3, target3);
    println!("Test 3 result: {}", result3 == None);

    let arr4 = vec![1, 2, 3, 4, 5, 6];
    let target4 = 1;
    let result4 = binary_search(&arr4, target4);
    println!("Test 4 result: {}", result4 == Some(0));

    let arr5 = vec![1, 2, 3, 4, 5, 6];
    let target5 = 6;
    let result5 = binary_search(&arr5, target5);
    println!("Test 5 result: {}", result5 == Some(5));
}
