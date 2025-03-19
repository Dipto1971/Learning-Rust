
// Q: Write a function that takes two strings as an input and returns the bigger amongst them
fn longest(a: String, b:String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("Small");
    {
        let str2 = String::from("Longer");
        longest_str= longest(str1, str2);
    }
    println!("{}", longest_str);
    // Eventually str2's data will be in longest_str. str2 goes out of scope
    // But longest_str does not, so the code will remain valid
}