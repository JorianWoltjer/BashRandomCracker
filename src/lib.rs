use crossbeam_channel::Sender;
use rayon::prelude::*;

use random::Random;

pub mod cli;
pub mod random;

pub trait CertainCracker {
    fn new(target: [u16; 3]) -> Self;
    fn find(&self) -> Option<u32>;
}
// Split `old` boolean into separate structs for performance reasons
pub struct New3Cracker {
    target: [u16; 3],
}
impl CertainCracker for New3Cracker {
    fn new(target: [u16; 3]) -> Self {
        Self { target }
    }

    fn find(&self) -> Option<u32> {
        // 30 bits
        (0..=u32::MAX / 4).into_par_iter().find_any(|&i| {
            let mut rng = Random::new(i, false);

            rng.next_16() == self.target[0]
                && rng.next_16() == self.target[1]
                && rng.next_16() == self.target[2]
        })
    }
}

pub struct Old3Cracker {
    target: [u16; 3],
}
impl CertainCracker for Old3Cracker {
    fn new(target: [u16; 3]) -> Self {
        Self { target }
    }

    fn find(&self) -> Option<u32> {
        // 31 bits
        (0..=u32::MAX / 2).into_par_iter().find_any(|&i| {
            let mut rng = Random::new(i, true);

            rng.next_16() == self.target[0]
                && rng.next_16() == self.target[1]
                && rng.next_16() == self.target[2]
        })
    }
}

pub trait UncertainCracker {
    fn new(target: [u16; 2]) -> Self;
    fn find(&self, tx: &Sender<(u32, bool)>);
}

pub struct New2Cracker {
    target: [u16; 2],
}
impl UncertainCracker for New2Cracker {
    fn new(target: [u16; 2]) -> Self {
        Self { target }
    }

    fn find(&self, tx: &Sender<(u32, bool)>) {
        (0..=u32::MAX / 4).into_par_iter().for_each(|i| {
            let mut rng = Random::new(i, false);

            if rng.next_16() == self.target[0] && rng.next_16() == self.target[1] {
                tx.send((i, false)).unwrap();
            }
        });
    }
}

pub struct Old2Cracker {
    target: [u16; 2],
}
impl UncertainCracker for Old2Cracker {
    fn new(target: [u16; 2]) -> Self {
        Self { target }
    }

    fn find(&self, tx: &Sender<(u32, bool)>) {
        // 31 bits
        (0..=u32::MAX / 2).into_par_iter().for_each(|i| {
            let mut rng = Random::new(i, true);

            if rng.next_16() == self.target[0] && rng.next_16() == self.target[1] {
                tx.send((i, true)).unwrap();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn find_new() {
        let cracker = New3Cracker::new([24697, 15233, 8710]);
        assert_eq!(cracker.find(), Some(1337));
    }

    #[test]
    fn find_old() {
        let cracker = Old3Cracker::new([24879, 21848, 15683]);
        assert_eq!(cracker.find(), Some(1337));
    }

    #[test]
    fn find_all_new() {
        let cracker = New2Cracker::new([20814, 24386]);
        let (tx, rx) = crossbeam_channel::unbounded();

        thread::spawn(move || {
            cracker.find(&tx);
        });

        let mut results = rx.into_iter().map(|(n, _old)| n).collect::<Vec<_>>();
        results.sort_unstable();
        assert_eq!(results, vec![0, 123459876, 572750907]);
    }

    #[test]
    fn find_all_old() {
        let cracker = Old2Cracker::new([20034, 24315]);
        let (tx, rx) = crossbeam_channel::unbounded();

        thread::spawn(move || {
            cracker.find(&tx);
        });

        let mut results = rx.into_iter().map(|(n, _old)| n).collect::<Vec<_>>();
        results.sort_unstable();
        assert_eq!(
            results,
            vec![0, 123459876, 852022490, 1082141963, 2040824050, 2147483647]
        );
    }
}
