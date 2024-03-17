use dyn_clone::DynClone;

pub trait Car: DynClone {
    fn meow(&self) -> &str;
}

dyn_clone::clone_trait_object!(Car);

// All cars must be of same type (struct is constrained to single generic type)
pub struct FixedCollegiate<T> {
    pub cars: Vec<Box<T>>,
}

impl<T> FixedCollegiate<T>
where
    T: Car,
{
    pub fn sing(&self) -> String {
        let mut a: Vec<&str> = Vec::new();
        for car in self.cars.iter() {
            a.push(car.meow());
        }
        String::from(a.join(" "))
    }
}

// All cars must have a type implementing the Car trait (struct is constrained to have a specific trait)
#[derive(Clone)]
pub struct Collegiate {
    pub cars: Vec<Box<dyn Car>>,
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

#[derive(Clone)]
pub struct Gnarpy<'a> {
    pub insults: Vec<&'a str>,
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
        let i: u32 = (10.0 * fractional) as u32;
        self.insults[i as usize]
    }
}

// TODO:
#[derive(Clone)]
pub struct BuhCat {}

impl Car for BuhCat {
    fn meow(&self) -> &str {
        "buh"
    }
}
