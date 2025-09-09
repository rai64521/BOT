use std::f64::consts::PI;


trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}


struct Circle {
    radius: f64,
}


impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn name(&self) -> String {
        "Circle".to_string()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}


impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> String {
        "Rectangle".to_string()
    }
}


fn print_shape_info(shape: &dyn Shape) {
    println!("Shape: {}", shape.name());
    println!("Area: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 3.0 };

    print_shape_info(&circle);
    print_shape_info(&rectangle);
}
