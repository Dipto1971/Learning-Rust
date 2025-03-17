pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    username: String,
    email: String,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}

fn main () {
    let user = User {
        username: String::from("Dipto"),
        email: String::from("dipto.haq771@gmail.com"),
    };
    println!("{}", user.summarize());
}
// Traits are similar to interfaces in other languages. They define a set of methods that a type must implement in order to be 
// considered a member of that trait. In this example, the Summary trait has a single method, summarize, that must be implemented 
// by any type that wants to be considered a Summary. The User struct implements the Summary trait by providing an implementation 
// of the summarize method. The main function creates a User instance and calls the summarize method on it. The output is "John".