#![no_std]
mod test;
use core::{fmt, hash};

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub pos: u64,
    pub val: u64,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    UP = -4,
    LEFT = -1,
    RIGHT = 1,
    DOWN = 4,
}

impl Action {
    pub const VALUES: [Self; 4] = [Action::UP, Action::LEFT, Action::RIGHT, Action::DOWN];
}

fn get(i: u64, j: u64) -> u64 {
    i >> j * 4 & 0xF
}

impl State {
    pub fn new(val: u64) -> Self {
        Self {
            val,
            pos: (1..16).map(|i| i << 4 * get(val, i)).sum(),
        }
    }
    pub fn swap(self) -> Self {
        Self {
            pos: self.val,
            val: self.pos,
        }
        //valid, but might not be reachable
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
    fn getx(&self, ind: u64) -> u64 {
        self.val >> ind * 4 & 3
    }
    fn gety(&self, ind: u64) -> u64 {
        self.val >> ind * 4 + 2 & 3
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
    pub fn manhattan(&self, other: &Self) -> i8 {
        (1..16)
            .map(|i| {
                (self.getx(i) as i8 - other.getx(i) as i8).abs()
                    + (self.gety(i) as i8 - other.gety(i) as i8).abs()
            })
            .sum()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl hash::Hash for State {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
