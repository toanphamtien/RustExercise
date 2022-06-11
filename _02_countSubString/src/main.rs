/**
 * Rust Exercise
 * 02: count substring in file
 */
use std::fs;
use std::io;

fn main() {
    // read file
    let mut path = std::env::current_dir().unwrap();
    path.push("src/test.txt");
    let text = fs::read_to_string(path).expect("read file failed");

    // get input string
    let mut keyword = String::new();
    println!("Enter the Keyword: ");
    io::stdin().read_line(&mut keyword).expect("get keyword failed");
    // remove \n
    keyword.pop();

    // count string in text file
    let num = text.to_lowercase().matches(&keyword.to_lowercase()).count();
    println!("Numbers of {:?} in text is {}", keyword, num);
}
