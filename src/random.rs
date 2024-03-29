pub const BASH_RAND_MAX: u16 = 0x7fff; // 15 bits

pub struct Random {
    pub seed: u32,
    last: u16,
    /// If true, use the old algorithm from bash 5.0 and earlier (check with `bash --version`)
    is_old: bool,
}
impl Random {
    pub fn new(seed: u32, is_old: bool) -> Self {
        // TODO: support `long` seed input
        Self {
            seed,
            last: 0,
            is_old,
        }
    }

    pub fn next_16(&mut self) -> u16 {
        self.next_seed();

        let result = if self.is_old {
            // Bash 5.0 and earlier
            self.seed as u16 & BASH_RAND_MAX
        } else {
            // Bash 5.1 and later
            ((self.seed >> 16) ^ (self.seed & 0xffff)) as u16 & BASH_RAND_MAX
        };
        // Skip if same as last
        if result == self.last {
            self.next_16()
        } else {
            self.last = result;
            result
        }
    }
    pub fn next_16_n(&mut self, n: usize) -> Vec<u16> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.next_16());
        }
        result
    }

    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next_16();
        }
    }

    pub fn next_seed(&mut self) -> u32 {
        if self.seed == 0 {
            self.seed = 123459876;
        }
        let h: i32 = self.seed as i32 / 127773;
        let l: i32 = self.seed as i32 - (127773 * h);
        let t: i32 = 16807 * l - 2836 * h;
        self.seed = if t < 0 { t + 0x7fffffff } else { t } as u32;

        self.seed
    }

    pub fn next_seed_n(&mut self, n: usize) -> Vec<u32> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.next_seed());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_52_zero() {
        // $ bash5.2 -c 'RANDOM=0; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(0, false);
        assert_eq!(rng.next_16_n(3), vec![20814, 24386, 149]);
    }

    #[test]
    fn bash_52_n() {
        // $ bash5.2 -c 'RANDOM=1337; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(1337, false);
        assert_eq!(rng.next_16_n(3), vec![24697, 15233, 8710]);
    }

    #[test]
    fn bash_52_big() {
        // $ bash5.2 -c 'RANDOM=2147483646; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(2147483646, false);
        assert_eq!(rng.next_16_n(3), vec![16807, 10791, 19566]);
    }

    #[test]
    fn bash_50_zero() {
        // $ bash5.0 -c 'RANDOM=0; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(0, true);
        assert_eq!(rng.next_16_n(3), vec![20034, 24315, 12703]);
    }

    #[test]
    fn bash_50_n() {
        // $ bash5.0 -c 'RANDOM=1337; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(1337, true);
        assert_eq!(rng.next_16_n(3), vec![24879, 21848, 15683]);
    }

    #[test]
    fn bash_50_big() {
        // $ bash5.0 -c 'RANDOM=2147483646; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(2147483646, true);
        assert_eq!(rng.next_16_n(3), vec![15960, 17678, 21286]);
    }
}
