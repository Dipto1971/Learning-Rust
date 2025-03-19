mod utils;
use utils::traits::{Summary, notify};
use chrono::NaiveDate;
use utils::traits::Book;
use utils::traits::Movie;

fn main() {

    let book = Book {
        name: String::from("A Song of Ice & Fire"),
        creators: vec![String::from("George R. R. Martin"), String::from("Martin G R R")],
        description: String::from("A Song of Ice and Fire is a series of high fantasy novels by the American author George R. R. Martin. He began writing the first volume, A Game of Thrones, in 1991, and published it in 1996. Martin, who originally envisioned the series as a trilogy, has released five out of seven planned volumes."),
        pages: 250
    };
    println!("{}", book.summarize());
    println!("{}", book.summmarize_creator());
    println!("{}", book.summarize_description());

    let movie = Movie{
        name: String::from("The Shawshank Redemption"),
        creators: vec![String::from("Frank Darabont")],
        release_date: NaiveDate::from_ymd_opt(1994, 9, 23).expect("Invalid date"),
        description: String::from("Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency."),
    };
    notify(&movie);
}
