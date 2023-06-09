
use std::env::args;

use bash_randcrack::{Cracker, random::Random, LegacyCracker, NewCracker};

const LEGACY: bool = true;

fn main() {
    let mut args = args();
    args.next();

    let r1 = args.next().unwrap().parse().unwrap();
    let r2 = args.next().unwrap().parse().unwrap();
    let r3 = args.next().unwrap().parse().unwrap();

    // TODO: check if less than 2^15
    // TODO: make clap arguments for infinite values and legacy mode, and no stop on first match
    // TODO: by default, try non-legacy mode first, then legacy mode if no match

    let seed = if LEGACY { 
        let cracker = LegacyCracker::new([r1, r2, r3]);
        cracker.find().expect("Failed to find seed")
    } else {
        let cracker = NewCracker::new([r1, r2, r3]);
        cracker.find().expect("Failed to find seed")
    };
    println!("Found seed: {seed}");
    
    let mut rng = Random::new(seed, LEGACY);
    rng.skip(3);
    println!("Next 3 values: {:?}", rng.next_16_n(3));
}
