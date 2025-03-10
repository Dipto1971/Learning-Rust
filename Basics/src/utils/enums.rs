enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_around (direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}

// Enums with values
enum Shape {
    Rectangle (f32, f32),
    Circle (f32),
    Triangle (f32, f32, f32),
}

fn print_area (shape: Shape) {
    match shape {
        Shape::Rectangle (width, height) => println!("Area of the rectangle: {}", width * height),
        Shape::Circle (radius) => println!("Area of the circle: {}", 3.14 * radius * radius),
        Shape::Triangle (a, b, c) => {
            let s = (a + b + c) / 2.0;
            let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
            println!("Area of the triangle: {}", area);
        },
    }
}

pub fn initialize_enums(){
    println!("Enums");
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    move_around(up);
    move_around(down);
    move_around(left);
    move_around(right);

    let rectangle = Shape::Rectangle(10.0, 20.0);
    let circle = Shape::Circle(5.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    print_area(rectangle);
    print_area(circle);
    print_area(triangle);
}