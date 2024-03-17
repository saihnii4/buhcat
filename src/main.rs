use buhcat::{BuhCat, Collegiate, FixedCollegiate, Gnarpy};

pub mod buhcat;

fn main() {
    let gnarp = Gnarpy {
        insults: vec!["hi", "asd", "asd"],
    };
    let gnarp2 = Box::new(Gnarpy {
        insults: vec!["VEEBORP!", "GLORP!", "YOU VORPFLOP"],
    });
    let buh = BuhCat {};
    let coll = Collegiate {
        cars: vec![Box::new(gnarp.clone()), Box::new(buh)],
    };
    println!("{}", coll.sing());
    let coll2 = FixedCollegiate {
        cars: vec![Box::new(gnarp), gnarp2],
    };
    println!("{}", coll2.sing());
}
