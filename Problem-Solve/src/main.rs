use std::io;

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input into an integer
    let w: u32 = input.trim().parse().expect("Please enter a valid number");

    // Check if the weight can be divided into two even parts
    if w >= 4 && w % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}