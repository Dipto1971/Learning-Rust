pub fn check_even() {
    let mut is_even = true;
    // By default, variables in Rust are immutable, meaning their value cannot be modified after they are assigned.
    // But if you want to change a variable's value, you must make it mutable using the mut keyword.

    let arr:[i8 ; 5]  = [1, 2, 3, 4, 5]; //i8 means 8-bit signed integer

    for n in 0..arr.len() {
        if arr[n] % 2 == 0 {
            println!("The number at index {} is even.", n);
            is_even = true;
            println!("is_even: {}", is_even);
        } else {
            println!("The number at index {} is not even.", n);
            is_even = false;
            println!("is_even: {}", is_even);
        }   
    }
}