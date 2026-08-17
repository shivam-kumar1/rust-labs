fn binary_search(arr: [i32; 10], target: i32, left: usize, right: usize) -> isize {
    todo!()
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // sorted array
    let target = 7;
    let result = binary_search(arr, target, 0, arr.len() - 1);
    println!("Result: {}", result);
}
