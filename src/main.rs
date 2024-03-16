use buhcat::{Collegiate, Gnarpy, BuhCat};

pub mod buhcat;

fn main() {
    let gnarp = Gnarpy {
        insults: vec!["hi", "asd", "asd"],
    };
    let buh = BuhCat {};
    let coll = Collegiate { cars: vec![Box::new(gnarp), Box::new(buh)] };
    println!("{}", coll.sing());
}
