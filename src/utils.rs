use std::fs;

pub fn read_file(filepath: String) -> String {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    contents
}

pub fn smallest(mut numbers: Vec<i64>) -> i64{
    numbers.sort_unstable();
    numbers[0]
}