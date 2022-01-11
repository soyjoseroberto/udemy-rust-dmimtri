use std::fmt::Debug;

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn print_info(shape: impl Shape) {
    println!("The area is {}", shape.area());
}

pub fn traits_as_params() {
    let c = Circle { radius: 2.0 };
    print_info(c);

    
}