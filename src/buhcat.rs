pub trait Car {
    fn meow(&self) -> &str;
}

pub struct Collegiate {
    pub cars: Vec<Box<dyn Car>>
}

impl Collegiate {
    pub fn sing(&self) -> String {
        let mut a: Vec<&str> = Vec::new();
        for car in self.cars.iter() {
            a.push(car.meow());
        }
        String::from(a.join(" "))
    }
}

pub struct Gnarpy<'a> {
    pub insults: Vec<&'a str>
}

impl Car for Gnarpy<'_> {
    fn meow(&self) -> &str {
        "Vorp!"
    }
}

impl Gnarpy<'_> {
    pub fn speak(&self) -> &str {
        let rng: f64 = rand::random::<u8>().into();
        let fractional: f64 = rng / 256.0;
        let i: u32 = (10.0*fractional) as u32;
        self.insults[i as usize]
    }
}


pub struct BuhCat {}

impl Car for BuhCat {
    fn meow(&self) -> &str {
        "buh"
    }
}
