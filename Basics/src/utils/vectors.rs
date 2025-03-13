// Collections: Vec, HashMap, HashSet, BTreeMap, BTreeSet
// Debug Trait: {:?}, {:#?}
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

fn main() {
    let mut v: Vec<i32> = Vec::new();

    for i in 1..10 {
        v.push(i);
    }
    
    
    let even_vec = find_even(&v); // I am passing the reference of v

    println!("Even numbers are: ");
    print! ("{:?} ", even_vec);

    println!("Original vector: ");
    print! ("{:?} ", v); // Implementing a debug trait
    println!();
}

fn find_even (vec: &Vec <i32>) -> Vec <i32> {
    let mut even_vec: Vec<i32> = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            even_vec.push(*val); // As the reference is passed
        }
    }
    even_vec
} 