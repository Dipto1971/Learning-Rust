// Two more important enums
// Option<T> and Result<T, E>
// Option<T> is used when a value can be present or absent
// Result<T, E> is used when an operation can succeed or fail
// Error handling and null handling in Rust is done using these two enums

use std::fs::read_to_string;

fn main () {
    let result = read_to_string("a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file")
    }
}