#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// Dynamic dispatch
struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape {
  fn area(&self) -> f64;
}

impl Shape for Square {
  fn area(&self) -> f64
  {
     self.side * self.side
  }
}

impl Shape for Circle {
  fn area(&self) -> f64
  {
    self.radius * self.radius * std::f64::consts::PI
  }
}

fn main() {
    let a = 123;
    let b = "hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());

    // Dynamic dispatch
    let shapes:[& dyn Shape; 4] = [
    &Circle{radius: 1.0},
    &Square{side: 3.0},
    &Circle{radius: 2.0},
    &Square{side: 4.0}
  ];
  for (i, shape) in shapes.iter().enumerate()
  {
    println!("Shape #{} has area {}", i, shape.area());
  }
}
