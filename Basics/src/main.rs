mod utils;
use utils::strings::get_first_word;
use utils::fibonacci::fib;
use utils::structs::initialize_structs;


fn main() {

    let sentence = String::from("This is Dipto speaking!");
    let first_word = get_first_word(sentence);
    println!("The first word of the sentence is: {}", first_word);

    println! ("{}", fib(4));

    initialize_structs();
}
