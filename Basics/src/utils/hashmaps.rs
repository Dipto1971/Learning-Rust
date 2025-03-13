// Hashmaps: Key-Value pairs of data

use std::collections::HashMap;

fn main () {
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("Dipto"), 25);
    users.insert(String::from("Rahul"), 24);
    users.insert(String::from("Rahim"), 23);

    let first_user_age= users.get("Dipto"); // This returns an Option<&u32>
    match first_user_age {
        Some(age) => println!("Dipto's age is: {}", age),
        None => println!("No age found for Dipto"),
    }
}

// Hashmaps are implemented in a way that the keys and values are stored in the heap
// When we match the key, we get an Option enum, which is either Some or None
// We can use the match statement to check if the key exists or not
// We can also use the unwrap method to get the value directly