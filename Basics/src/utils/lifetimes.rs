
// Q: Write a function that takes two strings as an input and returns the bigger amongst them
fn longest(a: String, b:String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

fn longest_with_reference(a: &str, b:&str) -> &str{
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
//     Why Does This Fail?
// The function returns either a or b, both of which are borrowed references.
// However, Rust needs to know how long the returned reference will live.
// Since a and b have different lifetimes (potentially coming from different scopes), 
// the compiler cannot determine a single lifetime for the return value.
}

fn longest_lifetime() {
    
}

// fn main() {
//     let longest_str;
//     let str1 = String::from("Small");
//     {
//         let str2 = String::from("Longer");
//         longest_str= longest(str1, str2);
//     }
//     println!("{}", longest_str);
//     // Eventually str2's data will be in longest_str. str2 goes out of scope
//     // But longest_str does not, so the code will remain valid
// }

// But what if we pass the reference

fn main() {
    let ans;
    let str1 = String::from("Small");
    {
        let str2 = String::from("Longer");

        ans = longest_with_reference(&str1, &str2);
        // str2 is created inside an inner scope { ... }, but ans is assigned outside that scope.
        // When str2 goes out of scope, it is dropped, but ans still holds a reference to it, leading to a dangling reference.
    }
}