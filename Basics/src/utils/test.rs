pub fn test () {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}