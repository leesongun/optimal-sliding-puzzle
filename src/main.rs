use astar::{Action, Builder, State};
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct NodeInfo {
    heuristic: u8,
    node: State,
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.heuristic == other.heuristic {
            Some(self.node.pos.cmp(&other.node.pos))
        } else {
            Some(other.heuristic.cmp(&self.heuristic))
        }
    }
}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.heuristic == other.heuristic {
            self.node.pos.cmp(&other.node.pos)
        } else {
            other.heuristic.cmp(&self.heuristic)
        }
    }
}

fn heur(s: &State, t: &State) -> u8 {
    std::cmp::max(s.manhattan(t), s.inversion(t))
}

fn astar(s: &State, t: &State) -> u8 {
    use std::collections::{BinaryHeap, HashMap};
    let mut tosee = BinaryHeap::new();
    let mut dists: HashMap<u64, u8, Builder> = HashMap::with_hasher(Builder::default());
    tosee.push(NodeInfo {
        heuristic: heur(s, t),
        node: *s,
    });
    dists.insert(s.pos, 1);
    let mut count = 0;
    while let Some(node) = tosee.pop() {
        if *dists.get(&node.node.pos).unwrap() == 0 {
            continue;
        }
        count += 1;
        if count % 1000000 == 0 {
            println!("{}", count);
        }
        let pathlength = *dists.get(&node.node.pos).unwrap();
        if node.node == *t {
            println!("{}", count);
            return pathlength - 1;
        }
        for i in &Action::VALUES {
            if let Some(x) = node.node + *i {
                let d = heur(t, &x);
                if let Some(&prev) = dists.get(&x.pos) {
                    if prev <= pathlength + 1 {
                        continue;
                    }
                }
                dists.insert(x.pos, pathlength + 1);
                tosee.push(NodeInfo {
                    heuristic: pathlength + 1 + d,
                    node: x,
                });
            }
        }
        dists.insert(node.node.pos, 0);
    }
    255
}
fn main() {
    use rand::SeedableRng;
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let a = State::rand(&mut rng);
    println!("{}", astar(&a, &State::default()));
}
