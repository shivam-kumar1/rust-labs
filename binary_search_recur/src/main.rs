fn binary_search(arr: [i32; 10], target: i32, left: usize, right: usize) -> isize {
    let mid = (left + right) / 2;
    if left > right {
        return -1;
    }

    if arr[mid] == target {
        return mid as isize; // NP
    } else if arr[mid] < target {
        return binary_search(arr, target, mid + 1, right);
    }

    binary_search(arr, target, left, mid - 1)
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // sorted array
    let target = 7;
    let result = binary_search(arr, target, 0, arr.len() - 1);
    println!("Result: {}", result);
}
