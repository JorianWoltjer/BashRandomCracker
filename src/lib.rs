const BASH_RAND_MAX: u16 = 0x7fff;  // 16 bits

pub struct Random {
    seed: u32,
    last: u16,
    /// If true, use the legacy algorithm from bash 5.0 and earlier (check with `bash --version`)
    legacy: bool,
}
impl Random {
    #[inline]
    pub fn new(seed: u32, legacy: bool) -> Self {
        Self { seed, last: 0, legacy }
    }

    #[inline]
    pub fn next_16(&mut self) -> u16 {
        self.seed = self.next_32();

        let result = if self.legacy { 
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

    #[inline]
    fn next_32(&mut self) -> u32 {
        // TODO: make ret = self.seed and move iterating code here
        let mut ret = if self.seed != 0 { self.seed } else { 123459876 };
        let h: i32 = ret as i32 / 127773;
        let l: i32 = ret as i32 - (127773 * h);
        let t: i32 = 16807 * l - 2836 * h;
        ret = if t < 0 { t + 0x7fffffff } else { t } as u32;

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_52_zero() {
        // $ bash5.2 -c 'RANDOM=0; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(0, false);
        assert_eq!(rng.next_16(), 20814);
        assert_eq!(rng.next_16(), 24386);
        assert_eq!(rng.next_16(), 149);
    }

    #[test]
    fn bash_52_n() {
        // $ bash5.2 -c 'RANDOM=1337; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(1337, false);
        assert_eq!(rng.next_16(), 24697);
        assert_eq!(rng.next_16(), 15233);
        assert_eq!(rng.next_16(), 8710);
    }

    #[test]
    fn bash_52_big() {
        // $ bash5.2 -c 'RANDOM=2147483646; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(2147483646, false);
        assert_eq!(rng.next_16(), 16807);
        assert_eq!(rng.next_16(), 10791);
        assert_eq!(rng.next_16(), 19566);
    }

    #[test]
    fn bash_50_zero() {
        // $ bash5.0 -c 'RANDOM=0; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(0, true);
        assert_eq!(rng.next_16(), 20034);
        assert_eq!(rng.next_16(), 24315);
        assert_eq!(rng.next_16(), 12703);
    }

    #[test]
    fn bash_50_n() {
        // $ bash5.0 -c 'RANDOM=1337; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(1337, true);
        assert_eq!(rng.next_16(), 24879);
        assert_eq!(rng.next_16(), 21848);
        assert_eq!(rng.next_16(), 15683);
    }

    #[test]
    fn bash_50_big() {
        // $ bash5.0 -c 'RANDOM=2147483646; echo $RANDOM $RANDOM $RANDOM'
        let mut rng = Random::new(2147483646, true);
        assert_eq!(rng.next_16(), 15960);
        assert_eq!(rng.next_16(), 17678);
        assert_eq!(rng.next_16(), 21286);
    }
}
