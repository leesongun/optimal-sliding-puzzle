use super::Action;

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub pos: u64,
    pub val: u64,
}

const fn get(i: u64, j: u64) -> u64 {
    i >> j * 4 & 0xF
}

impl State {
    #[must_use]
    pub fn new(val: u64) -> Self {
        Self {
            val,
            pos: (1..16).map(|i| i << 4 * get(val, i)).sum(),
        }
    }
    #[cfg(feature = "rand")]
    pub fn rand(rng: &mut impl rand::Rng) -> Self {
        let mut parity = true;
        let mut arr = [0; 16];
        for i in 1..16 {
            let j = rng.gen_range(0..=i);
            parity ^= j != i;
            arr[i] = arr[j];
            arr[j] = i;
        }
        parity ^= arr[0] & 1 != arr[0] & 4;
        if !parity {
            arr.swap(14, 15);
        }
        Self {
            val: (1..16).map(|i| (i as u64) << 4 * arr[i]).sum(),
            pos: (0..16).map(|i| (arr[i] as u64) << 4 * i).sum(),
        }
    }
    pub fn verify(&self) -> bool {
        for i in 0..16 {
            if get(self.val, get(self.pos, i)) != i {
                return false;
            }
        }
        true
    }
    pub fn act(&mut self, a: Action) -> bool {
        let curpos = get(self.pos, 0);
        let posdiff = a as u64;
        let tarpos = curpos.wrapping_add(posdiff);
        if tarpos >= 16
            || curpos & 3 == 0 && a == Action::LEFT
            || curpos & 3 == 3 && a == Action::RIGHT
        {
            return false;
        }
        let tarval = get(self.val, tarpos);
        self.pos = self
            .pos
            .wrapping_add(posdiff)
            .wrapping_sub(posdiff << 4 * tarval);
        self.val ^= tarval << 4 * curpos ^ tarval << 4 * tarpos;
        true
    }
    /// manhattan distance
    #[must_use]
    pub const fn manhattan(&self, other: &Self) -> u8 {
        //should be incrementally updated?
        let xs = self.pos & 0x3333_3333_3333_3330;
        let ys = self.pos & 0xCCCC_CCCC_CCCC_CCC0;
        let xo = other.pos & 0x3333_3333_3333_3330;
        let yo = other.pos & 0xCCCC_CCCC_CCCC_CCC0;
        let xs = xs | xs >> 1 & xs >> 2;
        let ys = ys | ys >> 1 & ys >> 2;
        let xo = xo | xo >> 1 & xo >> 2;
        let yo = yo | yo >> 1 & yo >> 2;
        let xs = xs | xs >> 1 & 0x1111_1111_1111_1111;
        let ys = ys | ys >> 1 & 0x4444_4444_4444_4444;
        let xo = xo | xo >> 1 & 0x1111_1111_1111_1111;
        let yo = yo | yo >> 1 & 0x4444_4444_4444_4444;
        ((xs ^ xo).count_ones() + (ys ^ yo).count_ones()) as u8
    }
    /// inversion distance
    pub fn inversion(&self, other: &Self) -> u8 {
        //should be incrementally updated?
        let mut vert = 0;
        let mut horz = 0;
        let s = (self.pos & 0x3333_3333_3333_3333) << 2 | (self.pos & 0xCCCC_CCCC_CCCC_CCCC) >> 2;
        let o = (other.pos & 0x3333_3333_3333_3333) << 2 | (other.pos & 0xCCCC_CCCC_CCCC_CCCC) >> 2;
        //while O(n lg n) and even O(n sqrt lg n) algorithms are available,
        //we just count inversions naively
        for i in 1..16 {
            for j in i..16 {
                if (get(self.pos, i) > get(other.pos, j)) ^ (get(self.pos, j) > get(other.pos, j)) {
                    vert += 1
                };
                if (get(s, i) > get(s, j)) ^ (get(o, j) > get(o, j)) {
                    horz += 1
                };
            }
        }
        vert / 3 + vert % 3 + horz / 3 + horz % 3
    }
    /// aditive pattern database
    /// decompose to "horizontal" and "vertical" moves
    pub fn walking(&self) -> u8 {
        unimplemented!()
    }
    //also provide update infos
    /*
    pub fn moves<B,F,R>(self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self) -> R,
        R: core::ops::Try<Ok = B>,
    {
        unimplemented!()
    }*/
}

impl core::fmt::Display for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for i in 0..16 {
            write!(f, "{:2}", get(self.val, i))?;
            if i & 3 == 3 {
                writeln!(f,)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for State {}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: 0xFEDCBA9876543210,
            val: 0xFEDCBA9876543210,
        }
    }
}

impl core::hash::Hash for State {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.pos);
    }
}

impl core::ops::Add<Action> for State {
    type Output = Option<Self>;
    fn add(mut self, a: Action) -> Self::Output {
        if self.act(a) {
            Some(self)
        } else {
            None
        }
    }
}

impl core::ops::AddAssign<Action> for State {
    fn add_assign(&mut self, a: Action) {
        self.act(a);
    }
}
