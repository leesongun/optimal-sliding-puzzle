use crate::*;

pub fn idastar_bpmx(s: &State, t: &State, h: &impl Fn(&State, &State) -> u8) -> u8 {
    let mut d = h(s, t);
    loop {
        let r = search_bpmx(*s, t, 0, d, h);
        if r == 0 {
            return d;
        } else {
            d = r + 2;
            //theoretically 1, but we know parity in this case
        }
    }
}

//transposition table?
fn search_bpmx(s: State, t: &State, p: u64, d: u8, h: &impl Fn(&State, &State) -> u8) -> u8 {
    if s == *t {
        return 0;
    }
    let l = h(&s, t);
    if l > d {
        return l;
    }
    for i in &Action::VALUES {
        if let Some(x) = s + *i {
            if x.val == p {
                continue;
            }
            let t = search_bpmx(x, t, s.val, d - 1, h);
            if t == 0 {
                return 0;
            }
            if t - 1 > d {
                return t - 1;
            }
        }
    }
    d
}
