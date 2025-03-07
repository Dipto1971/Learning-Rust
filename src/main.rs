mod strings;
mod test;

use strings::greet;
use strings::get_first_word;

mod conditionals;
use conditionals::check_even;

fn main() {

    let sentence = String::from("This is Dipto speaking!");
    let first_word = get_first_word(sentence);
    println!("The first word of the sentence is: {}", first_word);
}
