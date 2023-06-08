
use std::env::args;

use bash_randcrack::Random;

fn main() {
    let mut args = args();
    args.next();

    let r1 = args.next().unwrap().parse().unwrap();
    let r2 = args.next().unwrap().parse().unwrap();
    let r3 = args.next().unwrap().parse().unwrap();

    // TODO: check if less than 2^15

    let mut i = 0;
    loop {
        let mut rng = Random::new(i, true);

        if rng.next() == r1 && rng.next() == r2 && rng.next() == r3 {
            println!("Found seed: {}", i);
            println!("Next 3 values: [{}, {}, {}]", rng.next(), rng.next(), rng.next());
            break;
        }

        i += 1;
    }
}
