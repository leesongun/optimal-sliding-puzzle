use super::super::*;
use core::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Node(u8, State);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.0, self.1.pos).partial_cmp(&(self.0, other.1.pos))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.0, self.1.pos).cmp(&(self.0, other.1.pos))
    }
}

pub fn astar(s: &State, t: &State, h: &impl Fn(&State, &State) -> u8) -> u8 {
    use std::collections::{BinaryHeap, HashMap};
    let mut tosee = BinaryHeap::new();
    let mut dists: HashMap<u64, u8, Builder> = HashMap::with_hasher(Builder::default());
    tosee.push(Node(h(s, t), *s));
    //actually it is enough to store first 56 bits
    dists.insert(s.pos, 1);
    let mut count = 0;
    while let Some(Node(_, state)) = tosee.pop() {
        let pathlength = *dists.get(&state.pos).unwrap();
        if pathlength == 0 {
            continue;
        }
        count += 1;
        if state == *t {
            println!("{}", count);
            return pathlength - 1;
        }
        for i in &Action::VALUES {
            if let Some(x) = state + *i {
                let d = h(t, &x);
                if let Some(&prev) = dists.get(&x.pos) {
                    if prev <= pathlength + 1 {
                        continue;
                    }
                }
                dists.insert(x.pos, pathlength + 1);
                tosee.push(Node(pathlength + 1 + d, x));
            }
        }
        dists.insert(state.pos, 0);
    }
    255
}
