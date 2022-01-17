use std::rc::Rc;

struct Employee {
    name: Rc<String>
}

impl Employee {
    fn new(name: Rc<String>) -> Employee {
        Employee { name: name }
    }

    fn greet(&self) {
        println!("Employee's name is {}", self.name)
    }
}

pub fn run() {
    let name = Rc::new("Jose".to_string());
    println!("Employee = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    // new scope, vars will go out of scope soon
    {
        let employee = Employee::new(name.clone()); // Allows access to the varible and increments ref count.
        println!("Employee = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        employee.greet();
    }
    // Ref count would be back to 1 here
    println!("Employee = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    
}