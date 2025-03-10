struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
    fn perimeter (&self) -> i32 {
        return 2 * (self.width + self.height);
    }
    fn can_hold (&self, other: &Rect) -> bool {
        return self.width > other.width && self.height > other.height;
        // This is a method that takes another instance of Rect as a parameter
        // and returns a boolean value based on whether the other Rect can fit inside the instance Rect
    }
    fn square (size: i32) -> Rect {
        return Rect { width: size, height: size };
        // This is an associated function that returns a new instance of Rect
        // It is called with the struct name and not an instance of the struct
    }
}


pub fn initialize_structs() {
    let rect1 = Rect { width: 30, height: 50 };
    let rect2 = Rect { width: 10, height: 40 };
    let rect3 = Rect { width: 60, height: 45 };

    println!("The area of the rectangle is: {}", rect1.area());
    println!("The perimeter of the rectangle is: {}", rect1.perimeter());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rect::square(10);
    println!("The area of the square is: {}", square.area());
    println!("The perimeter of the square is: {}", square.perimeter());
}