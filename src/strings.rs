pub fn greet () {
    let greetings = String::from("Hello, World!");
    println!("{}", greetings);

    let char1 = greetings.chars().nth(0); 
    // This returns not the character but an Option type
    // Means it can be either Some or None, so I need to handle both cases
    match char1 {
        Some(c) => println!("The first character of the greetings is: {}", c),
        None => println!("There is no first character of the greetings."),
    }

    // Doing the same in the loop
    for n in 0..greetings.len() {
        match greetings.chars().nth(n) {
            Some(c) => println!("The character at index {} is: {}", n, c),
            None => println!("There is no character at index {}.", n),
        }
    }
}

pub fn get_first_word( sentence: String) -> String {
    let mut word = String::from("");
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        word.push(c);
    }
    return word;
}