// mod.rs is a special file that tells Rust that the directory is a module
// This allows us to split our code into multiple files
// We can then use the mod keyword to bring the code into scope
// When we create a new folder path, we need to create a mod.rs file to tell Rust that the directory is a module
pub mod strings;
pub mod fibonacci;
pub mod conditionals;
pub mod structs;
pub mod test;
pub mod enums;
pub mod option_enum;
pub mod error_enums;
pub mod chrono_implementation;
pub mod moving;
pub mod borrowing_better;
pub mod borrowing_ugly;
pub mod vectors;
pub mod vector_hashmap;
pub mod hashmaps;
pub mod iterators;
pub mod strings_slices;
pub mod generics;