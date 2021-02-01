#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn default() -> Person {
        Person::new("John Doe".to_string(), 32)
    }

    fn with_age(mut self, age: u8) -> Person {
        self.age = age;
        self
    }

    fn with_name(mut self, name: String) -> Person {
        self.name = name;
        self
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

pub fn use_person() {
    let john = Person::default()
        .with_age(52)
        .with_name("Téo".to_string())
        .with_age(23);
    let mut john2 = john;

    john2.set_age(32);

    // ne compile pas car john2 n'a pas la propriété Display
    // (il a seulement Debug).
    // println!("{}", john2);

    //ne compile pas car john a été move.
    // println!("{:?}", john);

    println!("{:#?}", john2);
}
