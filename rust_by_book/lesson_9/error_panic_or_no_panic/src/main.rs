struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        println!("{}", self.value);
        self.value
    }
}

fn main() {
    let guess1 = Guess::new(7);
    guess1.value();

    let guess2 = Guess::new(200);
    guess2.value();
}
