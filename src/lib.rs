const BASH_RAND_MAX: u16 = 0x7fff;  // 16 bits

pub struct Random {
    seed: u32,
    /// If true, use the legacy algorithm from bash 5.0 and earlier.
    legacy: bool,
}
impl Random {
    #[inline]
    pub fn new(seed: u32, legacy: bool) -> Self {
        Self { seed, legacy }
    }

    #[inline]
    pub fn next(&mut self) -> u16 {
        // TODO: skip if same as last
        
        self.seed = self.raw_next();

        if self.legacy { 
            self.seed as u16 & BASH_RAND_MAX
        } else { 
            ((self.seed >> 16) ^ (self.seed & 0xffff)) as u16 & BASH_RAND_MAX
        }
    }

    #[inline]
    fn raw_next(&mut self) -> u32 {
        let mut ret = if self.seed != 0 { self.seed } else { 123459876 };
        let h: i32 = ret as i32 / 127773;
        let l: i32 = ret as i32 - (127773 * h);
        let t: i32 = 16807 * l - 2836 * h;
        ret = if t < 0 { t + 0x7fffffff } else { t } as u32;

        ret
    }
}
