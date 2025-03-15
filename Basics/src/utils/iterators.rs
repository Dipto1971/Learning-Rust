// Iterators are a way to loop over a collection of items, such as an array or a vector.
use std::iter::Iterator;

// Iterating using for loops
fn iterating_using_for_loops() {
    let numbers = vec![1, 2, 3, 4, 5];
    let number_iter = numbers.iter();

    for number in number_iter {
        println!("Number: {}", number);
    }
}

// Mutating using iterators
fn mutating_using_iterators() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut number_iter = numbers.iter_mut(); // Creating a mutable reference to the iterator

    for number in number_iter {
        *number += 1; // As mutable reference to the iterator is created, we can mutate the values
    }

    println!("{:?}", numbers);
}

// Using Iterator via '.next()' method
fn using_iterator_next_method() {
    println!("Using Iterator next() method: ");
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut number_iter = numbers.iter_mut();

    // Using the next() method to get the next value
    // next() returns an Option, which is an enum that can either be Some or None
    // If the iterator has a value, it will return Some(value)
    // If the iterator has no value, it will return None
    match number_iter.next() {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    // We can also use a loop to iterate over the iterator
    // This will keep calling next() until it returns None
    for number in number_iter {
        println!("Number: {}", number);
    }

    // Create a new iterator for the while loop
    let mut number_iter = numbers.iter();
    // We can also use a while loop to iterate over the iterator
    // This will keep calling next() until it returns None
    while let Some(number) = number_iter.next() {
        println!("Value: {}", number);
    }
}

// Into Iterator: This trait is used to convert collections into an iterator that takes ownership of the collection
// This is useful when you want to consume the collection(vector) and don't need it anymore -> performance optimization
// For example, you can use into_iter() to consume a vector and get an iterator that takes ownership of the vector
fn into_iterator() {
    let numbers = vec![1, 2, 3, 4, 5];
    let number_iter = numbers.into_iter(); // Now I can no longer use the numbers vector

    for number in number_iter {
        println!("Number: {}", number);
    }
}


fn main () {
    iterating_using_for_loops();
    mutating_using_iterators();
    using_iterator_next_method();
}

// Three types of iterators in Rust:
// 1. Iter: This is the most common iterator and is used to iterate over a collection without modifying it.
// 2. IterMut: This is used to iterate over a collection and modify it.
// 3. IntoIter: This is used to consume a collection and get an iterator that takes ownership of the collection.