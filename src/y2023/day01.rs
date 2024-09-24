use std::fs;

pub fn read_lines(filename: &str) {
    let contents = fs::read_to_string(filename).expect("File not found");

    for line in contents.lines() {
        println!("{}", line)
    }
}
