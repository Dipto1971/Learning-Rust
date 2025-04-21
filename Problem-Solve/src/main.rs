use std::{clone, io};
mod utils{
    pub mod two_sum;
}
use utils::two_sum::two_sum;

fn main() {
    let mut input: Vec<i32> = [2,7,11,15].to_vec();
    let target: i32 = 9;
    let result = two_sum(input.clone(), target);
    print!("Result: {:?}\n", result);
}