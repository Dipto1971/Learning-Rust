fn main () {
    let s1 = String::from("Hello");
    do_something(&s1); // Passing the reference of s1 -> borrowing
    println!("{}", s1); // s1 is still valid
}

fn do_something(s: &String) {
    println!("{}", s); // 
}
// This is the best practice in Rust, this is borrowing
// This is a way to pass a reference, now the function do_something borrows the ownership of s1