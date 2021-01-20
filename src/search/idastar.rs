use crate::*;

pub fn idastar(s: &State, t: &State, h: &impl Fn(&State, &State) -> u8) -> u8 {
    let mut d = h(s, t);
    loop {
        let x = search(*s, t, 0, d, &h);
        if x == 0 {
            return d;
        } else {
            d = x;
        }
        /*
        if d == 255 {
            panic!()
        }*/
    }
}

fn search(s: State, t: &State, p: u64, d: u8, h: &impl Fn(&State, &State) -> u8) -> u8 {
    if s == *t {
        return 0;
    }
    let l = h(&s, t);
    if l > d {
        return l;
    }
    let mut min: u8 = 254;
    for i in &Action::VALUES {
        if let Some(x) = s + *i {
            if x.val == p {
                continue;
            }
            let t = search(x, t, s.val, d - 1, h);
            if t == 0 {
                return 0;
            }
            min = core::cmp::min(min, t);
        }
    }
    min + 1
}
