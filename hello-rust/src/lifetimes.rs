// &'static str; example of a lifetime. static means: live as long as the app

struct Person {
    name: String
}

// Lifetimes are represented by an apostrophe and then a name
struct Company<'z> {
    name: String,
    // Without a lifetime, the compiler would throw an error
    ceo: &'z Person
}

struct Robot<'a> {
    name: &'a str // Without the lifetime specifier you get errors
}

impl<'a> Robot<'a> { // Notice how the lifetime is specified in 2 places
    fn talk(&self) {
        println!("I am a robot and my name is {}", self.name)
    }
}

pub fn run() {
    let boss = Person { name: String::from("Elon Musk")};
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    println!("Company: {}, CEO: {}", tesla.name, tesla.ceo.name);

    // Lifetime in structure
    let robot = Robot { name: "Optimus Prime" };
    robot.talk();
}