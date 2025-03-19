
use chrono::NaiveDate;

/// The `Summary` trait defines a set of methods that must be implemented for a type to provide
/// a summary of its content. This trait is useful for types that need to provide a concise
/// description or summary of their data.
///
/// # Required Methods
///
/// - `summarize(&self) -> String`: Provides a summary of the content.
/// - `summarize_author(&self) -> String`: Provides a summary of the author(s).
/// - `summarize_description(&self) -> String`: Provides a summary of the description.

pub trait Summary {
    fn summarize(&self) -> String;
    fn summmarize_creator(&self) -> String;
    fn summarize_description(&self) -> String;
}

pub struct Book {
    pub name: String,
    pub creators: Vec<String>,
    pub description: String,
    pub pages: u32
}

pub struct Movie {
    pub name: String,
    pub creators: Vec<String>,
    pub release_date: NaiveDate,
    pub description: String,
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
    fn summmarize_creator(&self) -> String {
        let creator = self.creators.join(", ");
        return creator;
    }
}

impl Summary for Movie {
    fn summarize(&self) -> String {
        let summary = format!("{} directed by {}", self.name, self.creators.join(", "));
        return summary;
    }
    fn summarize_description(&self) -> String {
        let summary = self.description.clone();
        return summary;
    }
    fn summmarize_creator(&self) -> String {
        let creator = self.creators.join (" ,");
        return creator;
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
