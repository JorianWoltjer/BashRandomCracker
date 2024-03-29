use std::thread;

use clap::Parser;
use crossbeam_channel::unbounded;

use bashrand::{
    cli::{Args, SubCommands, Version},
    log,
    random::{Random, BASH_RAND_MAX},
    CollisionCracker, MultiResultCracker, New2Cracker, New3Cracker, Old2Cracker, Old3Cracker,
    OneResultCracker, Result,
};

fn main() {
    let args = Args::parse();

    if let Err(e) = do_main(args) {
        log::error(e);
    }
}

fn print_seed_and_clone(seed: u32, skip: usize, is_old: bool, number: usize) {
    println!(
        "Seed: {seed}{} ({})",
        match skip {
            0 => String::from(""),
            _ => format!(" +{skip}"),
        },
        if is_old { "old" } else { "new" }
    );

    let mut rng = Random::new(seed, is_old);
    rng.skip(skip);

    match number {
        0 => (),
        1 => println!("  Next value: {}", rng.next_16()),
        _ => println!("  Next {} values: {:?}", number, rng.next_16_n(number)),
    }
}

fn do_main(args: Args) -> Result<()> {
    let (version_old, version_new) = match args.version {
        Version::Old => (true, false),
        Version::New => (false, true),
        Version::Both => (true, true),
    };

    match args.command {
        SubCommands::Crack { numbers } => {
            if numbers.iter().any(|n| *n > BASH_RAND_MAX) {
                return Err(
                    format!("Numbers must be at most 15 bits (max: {})", BASH_RAND_MAX).into(),
                );
            };

            match numbers.len() {
                // Certain (one possible seed)
                3 => {
                    let numbers = [numbers[0], numbers[1], numbers[2]];

                    log::progress("Searching for seeds...".to_string());

                    let (mut seed, mut is_old) = (None, false);

                    if version_new {
                        let cracker = New3Cracker::new(numbers);
                        seed = cracker.find();
                    }
                    if version_old && seed.is_none() {
                        let cracker = Old3Cracker::new(numbers);
                        seed = cracker.find();
                        is_old = true;
                    }

                    print_seed_and_clone(seed.ok_or("Couldn't find seed")?, 3, is_old, args.number);

                    log::success("Finished!");
                }
                // Uncertain (multiple possible seeds)
                2 => {
                    let numbers = [numbers[0], numbers[1]];

                    let (tx, rx) = unbounded();

                    log::progress("Searching for seeds...".to_string());

                    thread::spawn(move || {
                        if version_new {
                            let cracker = New2Cracker::new(numbers);
                            cracker.find(&tx);
                        }
                        if version_old {
                            let cracker = Old2Cracker::new(numbers);
                            cracker.find(&tx);
                        }
                    });

                    // Stream all found seeds
                    let mut count = 0;
                    for (seed, old) in rx {
                        print_seed_and_clone(seed, 2, old, args.number);
                        count += 1;
                    }

                    if count == 0 {
                        return Err("Couldn't find seed".into());
                    } else {
                        log::success(format!("Finished! ({count} seeds)"));
                    }
                }
                _ => unreachable!(),
            }
        }
        SubCommands::Get { seed, skip } => {
            if version_new {
                print_seed_and_clone(seed, skip, false, args.number);
            }
            if version_old {
                print_seed_and_clone(seed, skip, true, args.number);
            }
        }
        SubCommands::Seeds { seed } => {
            // Seed generation is the same for both versions
            let mut rng = Random::new(seed, false);
            let seeds = rng.next_seed_n(args.number);
            println!("Next {} seeds: {:?}", args.number, seeds);
        }
        SubCommands::Collide { n } => {
            let (tx, rx) = unbounded();

            log::progress("Searching for seeds...".to_string());

            thread::spawn(move || {
                let cracker = CollisionCracker::new(n);
                cracker.find(&tx);
            });

            // Stream all found seeds
            let mut count = 0;
            for seed in rx {
                println!("Seed: {seed}: {n}");
                count += 1;
            }

            if count == 0 {
                return Err("Couldn't find seed".into());
            } else {
                log::success(format!("Finished! ({count} seeds)"));
            }
        }
    }
    Ok(())
}
