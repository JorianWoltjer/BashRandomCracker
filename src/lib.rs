use std::error::Error;

use crossbeam_channel::Sender;
use rayon::prelude::*;

use random::Random;

pub mod cli;
pub mod log;
pub mod random;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait OneResultCracker {
    fn find(&self) -> Option<u32>;
}
// Split `old` boolean into separate structs for performance reasons
pub struct New3Cracker {
    target: [u16; 3],
}
impl New3Cracker {
    pub fn new(target: [u16; 3]) -> Self {
        Self { target }
    }
}
impl OneResultCracker for New3Cracker {
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
impl Old3Cracker {
    pub fn new(target: [u16; 3]) -> Self {
        Self { target }
    }
}
impl OneResultCracker for Old3Cracker {
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

pub trait MultiResultCracker {
    fn find(&self, tx: &Sender<(u32, bool)>);
}

pub struct New2Cracker {
    target: [u16; 2],
}
impl New2Cracker {
    pub fn new(target: [u16; 2]) -> Self {
        Self { target }
    }
}
impl MultiResultCracker for New2Cracker {
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
impl Old2Cracker {
    pub fn new(target: [u16; 2]) -> Self {
        Self { target }
    }
}
impl MultiResultCracker for Old2Cracker {
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

pub trait MultiResultVersionCracker {
    fn find(&self, tx: &Sender<u32>);
}

pub struct CollisionCracker {
    target: u16,
}
impl CollisionCracker {
    pub fn new(target: u16) -> Self {
        Self { target }
    }
}
impl MultiResultVersionCracker for CollisionCracker {
    fn find(&self, tx: &Sender<u32>) {
        (0..=u32::MAX / 4).into_par_iter().for_each(|i| {
            let mut rng_old = Random::new(i, true);
            // Setting RANDOM= already iterates once
            rng_old.next_16();

            if rng_old.next_16() == self.target {
                // Also check new version
                let mut rng_new = Random::new(i, false);
                rng_new.next_16();

                if rng_new.next_16() == self.target {
                    tx.send(i).unwrap();
                }
            }
        })
    }
}

pub struct New1Cracker {
    target: u16,
}
impl New1Cracker {
    pub fn new(target: u16) -> Self {
        Self { target }
    }
}
impl MultiResultVersionCracker for New1Cracker {
    fn find(&self, tx: &Sender<u32>) {
        (0..=u32::MAX / 4).into_par_iter().for_each(|i| {
            let mut rng = Random::new(i, false);
            // Setting RANDOM= already iterates once
            rng.next_16();

            if rng.next_16() == self.target {
                tx.send(i).unwrap();
            }
        });
    }
}

pub struct Old1Cracker {
    target: u16,
}
impl Old1Cracker {
    pub fn new(target: u16) -> Self {
        Self { target }
    }
}
impl MultiResultVersionCracker for Old1Cracker {
    fn find(&self, tx: &Sender<u32>) {
        (0..=u32::MAX / 2).into_par_iter().for_each(|i| {
            let mut rng = Random::new(i, true);
            // Setting RANDOM= already iterates once
            rng.next_16();

            if rng.next_16() == self.target {
                tx.send(i).unwrap();
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
