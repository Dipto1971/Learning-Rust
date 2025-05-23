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
    // Output: Number: 1 -> Number: 2 -> Number: 3 -> Number: 4 -> Number: 5
}

// Sum method: This method is used to sum all the values in an iterator
fn sum_method() {
    let numbers = vec![1, 2, 3, 4, 5]; 
    let numbers_iter = numbers.iter();
    let sum: i32 = numbers_iter.sum(); // Now I can no longer use the numbers_iter iterator, it has been consumed
    println!("Sum: {}", sum);
}

// Map method: This method is used to transform each value in an iterator
fn map_method() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    let new_numbers = numbers_iter.map(|x| x * 2); // Now I can no longer use the numbers_iter iterator, it has been consumed
    println!("{:?}", new_numbers);
}

// Filter method: This method is used to filter values in an iterator
fn filter_method() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    let new_numbers = numbers_iter.filter(|x| *x % 2 == 0).map(|x| x*2); // Now I can no longer use the numbers_iter iterator, it has been consumed
    
    let new_vec: Vec<i32> = new_numbers.collect();
    for i in new_vec {
        println!("{}", i);
    }
}

fn main () {
    filter_method();
}

// Three types of iterators in Rust:
// 1. Iter: This is the most common iterator and is used to iterate over a collection without modifying it.
// 2. IterMut: This is used to iterate over a collection and modify it.
// 3. IntoIter: This is used to consume a collection and get an iterator that takes ownership of the collection.


