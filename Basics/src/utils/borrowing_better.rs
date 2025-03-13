fn main () {
    let mut s1 = String::from("Hello");
    do_something(&mut s1); // Passing the reference of s1 -> borrowing
    println!("{}", s1); // s1 is still valid

    // let s2 = &s1; // This is also borrowing
    // println!("{}", s2);
    // println!("{}", s1);
}

fn do_something(s: &mut String) { // Making the S2 mutable
    s.push_str(" Dipto");
    print!("{}", s); // 
    println!(" at do_something funtion.");
}
// This is the best practice in Rust, this is borrowing
// This is a way to pass a reference, now the function do_something borrows the ownership of s1