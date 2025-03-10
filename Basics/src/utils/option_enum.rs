// Two more important enums
// Option<T> and Result<T, E>
// Option<T> is used when a value can be present or absent
// Result<T, E> is used when an operation can succeed or fail
// Error handling and null handling in Rust is done using these two enums

enum CustomOption {
    Some(i32),
    None,
}

// With enumarate
fn find_first_a (s: String) -> CustomOption {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None // this is similar to null in other languages
}

fn main () {
    let s = String::from("Hello, World!");
    let index = find_first_a(s);
    match index {
        CustomOption::Some(value) => println!("The index of the first 'a' is {}", value),
        CustomOption::None => println!("There is no 'a' in the string"),
    }
}