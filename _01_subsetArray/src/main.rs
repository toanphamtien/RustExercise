/**
 * Rust Excercise
 * 01: Is array a subset of another array?
 */
fn isSubArray(arr: &[i32], subArr: &[i32]) -> bool{
    if subArr.len() > arr.len() {
        return false;
    }

    let mut res: bool = false;

    for i in 0..arr.len() {
       if arr[i] == subArr[0] {
            println!("arr[{}] = {}", i, arr[i]);
            println!("subArr[0] = {}", subArr[0]);
            for j in 1..subArr.len() {
                if i + j >= arr.len() {
                    return false;
                }
                println!("arr[{}] = {}", i+j, arr[i+j]);
                println!("subArr[{}] = {}", j, subArr[j]);
                if arr[i+j] != subArr[j] {
                    break;
                }
                if j == subArr.len() - 1{
                    res = true;
                }
            }
       }
    }
    return res;
}

fn main() {
    let arr = [7, 3, 5, 56, 8, 7, 3, 56, 9];
    let subArr = [7, 3, 56];

    if isSubArray(&arr, &subArr) {
        println!("Is Subset");
    }
    else {
        println!("Is not Subset");
    }
}
