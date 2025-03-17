fn push_replace_str() {
    let mut name = String::from("Dipto");
    name.push_str(" Haque");
    println!("{}", name);
    println!("{} ", name.len());

    name.replace_range(8..name.len(), "");
    println!("{}", name);
    println!("{}", name.len());
}

// Q: Write a function that takes a string and returns the first word it finds in that string.
fn first_word(s:   String) -> String {
    let mut word = String::new();
    for c in s.chars() {
        if c == ' ' {
            break;
        }
        word.push(c);
    }
    word
    // This is not referrencing the original string, so if the original string changes, this will not change
    // To solve the problem, we can use slices
}

// View into the original string and not copy the value
// &str -> string slice
fn first_word_slice(s: &String) -> &str { // when I call the function, the value will be borrowed
    // let manual_slice = &s[0..4];
    // s.clear(); // manual_slice is pointing to the original string, so we cannot clear the original string-> dangling reference

    let mut space_index = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            space_index = i;
            break;
        }
    }
    return &s[0..space_index]; // returning a slice of the borrowed string
}

fn main () {
    push_replace_str();
    let s = String::from("Hello world!");
    let word = first_word_slice(&s);
    println!("{}", word); // Slice of the original string
    println!("{}", s); // Original string
}