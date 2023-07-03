fn binary_search(arr: &[i32], n: i32, k: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if arr[mid as usize] == k {
            return mid;
        } else if arr[left as usize] <= arr[mid as usize] {
            if arr[left as usize] <= k && k <= arr[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if arr[mid as usize] <= k && k <= arr[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n = arr.len() as i32;
    let k = 5;

    let result = binary_search(&arr, n, k);
    println!("Index of {}: {}", k, result);
}