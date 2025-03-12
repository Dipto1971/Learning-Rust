fn create_string() {
    let mut s1 = String::from("This is a string");
    s1 = print_str(s1); // returns s1, means I'm borrowing the ownership to the function

    println!("{}", s1); // Got back the ownership
}

fn print_str(s2: String) -> String {
    println!("{}", s2);
    s2 // returning the value means giving ownership back to the caller
}

fn main() {
    // call the function
    create_string();
}

// This is not best practice, but it is possible to do this in Rust
// This is not recommended because it is not clear who owns the data