use random::Random;

pub mod random;
pub mod cli;

pub trait Cracker {
    fn new(target: [u16; 3]) -> Self;
    fn find(&self) -> Option<u32>;
}

pub struct NewCracker {
    target: [u16; 3],
}
impl Cracker for NewCracker {
    fn new(target: [u16; 3]) -> Self {
        Self { target }
    }

    fn find(&self) -> Option<u32> {
        for i in 0..=u32::MAX {  // TODO: half this
            let mut rng = Random::new(i, false);

            if rng.next_16() == self.target[0] && rng.next_16() == self.target[1] && rng.next_16() == self.target[2] {
                return Some(i);
            }
        }
        None
    }
}

pub struct LegacyCracker {
    target: [u16; 3],
}
impl Cracker for LegacyCracker {
    fn new(target: [u16; 3]) -> Self {
        Self { target }
    }

    fn find(&self) -> Option<u32> {
        for i in 0..=u32::MAX {  // TODO: half this
            let mut rng = Random::new(i, true);

            if rng.next_16() == self.target[0] && rng.next_16() == self.target[1] && rng.next_16() == self.target[2] {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cracker() {
        let cracker = NewCracker::new([24697, 15233, 8710]);
        assert_eq!(cracker.find(), Some(1337));
    }

    #[test]
    fn legacy_cracker() {
        let cracker = LegacyCracker::new([24879, 21848, 15683]);
        assert_eq!(cracker.find(), Some(1337));
    }
}
