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

pub fn dibbs(s: &State, t: &State, h: &impl Fn(&State, &State) -> u8) -> u8 {
    use std::collections::hash_map::Entry;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    let mut queue = [BinaryHeap::new(), BinaryHeap::new()];
    queue[0].push(Node(h(s, t), *s));
    queue[1].push(Node(h(s, t), *t));

    //actually it is enough to store first 56 bits
    let mut dists = [
        HashMap::with_hasher(Builder::default()),
        HashMap::with_hasher(Builder::default()),
    ];
    dists[0].insert(s.pos, 0);
    dists[1].insert(t.pos, 0);

    let mut closed = [
        HashSet::with_hasher(Builder::default()),
        HashSet::with_hasher(Builder::default()),
    ];

    let mut UB = 100;
    let mut count = 0;

    while !queue[0].is_empty() && !queue[1].is_empty() {
        let a = queue[0].peek().unwrap().0;
        let b = queue[1].peek().unwrap().0;
        if a + b >= 2 * UB {
            break;
        }
        if a < b {
            let node = queue[0].pop().unwrap().1;

            if closed[0].contains(&node.pos) {
                continue;
            }
            count += 1;
            closed[0].insert(node.pos);
            let path = *dists[0].get(&node.pos).unwrap();

            for &i in &Action::VALUES {
                if let Some(x) = node + i {
                    match dists[0].entry(x.pos) {
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
                    if let Some(y) = dists[1].get(&x.pos) {
                        UB = std::cmp::min(UB, y + path + 1);
                    }
                    queue[0].push(Node(2 * path + h(&x, t) - h(s, &x), x));
                }
            }
        } else {
            let node = queue[1].pop().unwrap().1;

            if closed[1].contains(&node.pos) {
                continue;
            }
            count += 1;
            closed[1].insert(node.pos);
            let path = *dists[1].get(&node.pos).unwrap();

            for &i in &Action::VALUES {
                if let Some(x) = node + i {
                    match dists[1].entry(x.pos) {
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
                    if let Some(y) = dists[0].get(&x.pos) {
                        UB = std::cmp::min(UB, y + path + 1);
                    }
                    queue[1].push(Node(2 * path - h(&x, t) + h(s, &x), x));
                }
            }
        }
    }
    println! {"{}", count};
    UB
}
