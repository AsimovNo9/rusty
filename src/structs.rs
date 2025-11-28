// struct Color{
//     red: u8,
//     green: u8,
//     blue: u8,
// }


// pub fn run(){
//     let c: Color = Color{red: 255, green: 255, blue:255};

//     println!("Colors are: {} {} {}", c.red, c.green, c.blue)
// }

struct Person{
    first_name: String,
    last_name: String,
}

impl Person{
    fn new(first: &str, last: &str) -> Person {
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }

    fn full_name(&self)-> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }
}

pub fn run(){
    let mut p = Person::new("john","doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
}