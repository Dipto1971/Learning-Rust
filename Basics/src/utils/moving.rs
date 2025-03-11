fn create_string() {
    // ownership -> only one owner of a piece of data at a time
    // When a variable goes out of scope, Rust will automatically call drop function to free the memory
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); 
    // This will throw an error
    // This is because s1 has been moved to s2, Rust does not allow multiple ownership
    // This is a feature of Rust to prevent data from being copied multiple times
    // So, s1 is no longer valid in memory
}

fn clone_string() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn move_int() {
    let x = 5;
    let y = x; // This will not throw an error because integers are stored on the stack
    println!("x: {}, y: {}", x, y); 
}

fn main() {
    create_string();
    clone_string();
    move_int();
}