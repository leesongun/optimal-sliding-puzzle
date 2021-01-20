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
    use std::collections::hash_map::Entry;
    use std::collections::{BinaryHeap, HashMap};

    let mut queue = BinaryHeap::new();
    queue.push(Node(h(s, t), *s));

    //actually it is enough to store first 56 bits
    let mut dists = HashMap::with_hasher(Builder::default());
    dists.insert(s.pos, 1);

    let mut count = 0;
    while let Some(Node(_, state)) = queue.pop() {
        let path = dists.insert(state.pos, 0).unwrap();
        if path == 0 {
            continue;
        }
        count += 1;
        if state == *t {
            println!("{}", count);
            return path - 1;
        }
        for &i in &Action::VALUES {
            if let Some(x) = state + i {
                let d = h(t, &x);
                match dists.entry(x.pos) {
                    Entry::Occupied(mut entry) => {
                        if *entry.get() <= path + 1 {
                            continue;
                        }
                        entry.insert(path + 1);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(path + 1);
                    }
                }
                queue.push(Node(path + 1 + d, x));
            }
        }
    }
    255
}
