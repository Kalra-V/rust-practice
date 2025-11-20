enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    let circle = Shape::Circle(4.0);
    let square = Shape::Square(5.0);
    let rect = Shape::Rectangle(2.4, 6.8);

    
}

fn calculate_area(shape: Shape) -> f64 {
    let ans = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height ) => width * height
    };
    return ans;
}
