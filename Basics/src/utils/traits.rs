pub trait Summary {
    fn summarize(&self) -> String;
    fn summmarize_author(&self) -> String;
    fn summarize_description(&self) -> String;
}

struct Book {
    name: String,
    authors: Vec<String>,
    description: String,
    pages: u32
}

impl Summary for Book {
    fn summarize(&self) -> String {
        let summary = format!("{} {}",self.name, self.description);
        return summary;
    }
    fn summarize_description(&self) -> String {
        let summary = self.description.clone();
        return summary;
    }
    fn summmarize_author(&self) -> String {
        let author = self.authors.join(", ");
        return author;
    }
}

fn main () {
    let book = Book {
        name: String::from("A Song of Ice & Fire"),
        authors: vec![String::from("George R. R. Martin"), String::from("Martin G R R")],
        description: String::from("A Song of Ice and Fire is a series of high fantasy novels by the American author George R. R. Martin. He began writing the first volume, A Game of Thrones, in 1991, and published it in 1996. Martin, who originally envisioned the series as a trilogy, has released five out of seven planned volumes."),
        pages: 250
    };
    println!("{}", book.summarize())
}