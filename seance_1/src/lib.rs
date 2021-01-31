#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn default() -> Self {
        Self::new("John Doe".to_string(), 32)
    }

    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

pub fn use_person() {
    let john = Person::default().with_age(52).with_name("Téo".to_string()).with_age(23);
    let mut john2 = john;

    john2.set_age(32);

    // println!("{}", john); //ne compile pas car john a été move.
    println!("{:#?}", john2);
}