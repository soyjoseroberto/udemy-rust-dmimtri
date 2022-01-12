#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
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

// Three ways to specify an argument that implementes multiple traits
// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape + Debug>(shape: T) {
fn print_info<T>(shape: T) where T: Shape + Debug {
    println!("{:?}", shape); // {:?} is the debug output
    println!("The area is {}", shape.area());
}

pub fn traits_as_params() {
    let c = Circle { radius: 2.0 };
    print_info(c);

    
}