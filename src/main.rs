use std::thread;

use clap::Parser;
use crossbeam_channel::unbounded;

use bashrand::{
    cli::{Args, SubCommands, Version},
    random::Random,
    CertainCracker, New2Cracker, New3Cracker, Old2Cracker, Old3Cracker, UncertainCracker,
};

fn print_seed_and_clone(seed: u32, skip: usize, old: bool, number: usize) {
    println!(
        "Seed: {seed}{} ({})",
        match skip {
            0 => String::from(""),
            _ => format!(" +{skip}"),
        },
        if old { "old" } else { "new" }
    );

    let mut rng = Random::new(seed, old);
    rng.skip(skip);

    match number {
        0 => (),
        1 => println!("  Next value: {}", rng.next_16()),
        _ => println!("  Next {} values: {:?}", number, rng.next_16_n(number)),
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        SubCommands::Crack { numbers } => {
            let numbers = numbers
                .iter()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u16>>();

            match numbers.len() {
                // Certain (one possible seed)
                3 => {
                    let numbers = [numbers[0], numbers[1], numbers[2]];

                    let (seed, old) = match args.version {
                        Version::Old => {
                            let cracker = Old3Cracker::new(numbers);
                            (cracker.find().expect("Failed to find seed"), true)
                        }
                        Version::New => {
                            let cracker = New3Cracker::new(numbers);
                            (cracker.find().expect("Failed to find seed"), false)
                        }
                        Version::Both => {
                            // Try new first
                            let cracker = New3Cracker::new(numbers);
                            if let Some(seed) = cracker.find() {
                                (seed, false)
                            } else {
                                // If not found, try old
                                let cracker = Old3Cracker::new(numbers);
                                (cracker.find().expect("Failed to find seed"), true)
                            }
                        }
                    };

                    print_seed_and_clone(seed, 3, old, args.number);
                }
                // Uncertain (multiple possible seeds)
                2 => {
                    let numbers = [numbers[0], numbers[1]];

                    let (tx, rx) = unbounded();

                    thread::spawn(move || {
                        match args.version {
                            Version::Old => {
                                let cracker = Old2Cracker::new(numbers);
                                cracker.find(&tx);
                            }
                            Version::New => {
                                let cracker = New2Cracker::new(numbers);
                                cracker.find(&tx);
                            }
                            Version::Both => {
                                // Try new first
                                let cracker = New2Cracker::new(numbers);
                                cracker.find(&tx);
                                // Also try old
                                let cracker = Old2Cracker::new(numbers);
                                cracker.find(&tx);
                            }
                        }
                    });

                    // Stream all found seeds
                    for (seed, old) in rx {
                        print_seed_and_clone(seed, 2, old, args.number);
                    }
                }
                _ => unreachable!(),
            }
        }
        SubCommands::Get { seed, skip } => match args.version {
            Version::Old => print_seed_and_clone(seed, skip, true, args.number),
            Version::New => print_seed_and_clone(seed, skip, false, args.number),
            Version::Both => {
                print_seed_and_clone(seed, skip, true, args.number);
                print_seed_and_clone(seed, skip, false, args.number);
            }
        },
    }
}
