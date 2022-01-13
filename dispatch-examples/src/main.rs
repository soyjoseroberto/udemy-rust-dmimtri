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

fn main() {
    let a = 123;
    let b = "hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());
}
