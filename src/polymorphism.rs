trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("{} {}", item.description(), item.price())
}

fn vendor_text_dynamic(item: &dyn Sellable) -> String {
    format!("{} {}", item.description(), item.price())
}

struct Sword {
    name: String,
    damage: u16,
    swing_time_ms: u16,
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        self.damage * self.swing_time_ms
    }
    fn description(&self) -> String {
        format!("{}", self.name)
    }
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor * self.block
    }
    fn description(&self) -> String {
        format!("{}", self.name)
    }
}

struct Shield {
    name: String,
    armor: u16,
    block: u16,
}

pub fn run() {
    let sword = Sword {
        name: "first sword".to_string(),
        damage: 10,
        swing_time_ms: 1500,
    };
    let shield = Shield {
        name: "first shield".to_string(),
        armor: 50,
        block: 35,
    };

    println!("{}", vendor_text_static(&sword));
    println!("{}", vendor_text_static(&shield));

    let sellables: Vec<&dyn Sellable> = vec![&sword, &shield];

    for s in sellables {
        println!("{}", vendor_text_dynamic(s));
    }
}
