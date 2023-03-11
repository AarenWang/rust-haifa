use std::convert::Into;
#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    let person = Person::new("Alex");
    println!("{}, {:?}", person.name, person);
    let person = Person::new("Alex".to_string());
    println!("{}, {:?}", person.name, person);
}
