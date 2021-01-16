use astar::{Action, Builder, State};
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct NodeInfo {
    heuristic: u8,
    node: State,
}

impl PartialOrd for NodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.heuristic, self.node.pos).partial_cmp(&(self.heuristic, other.node.pos))
    }
}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.heuristic, self.node.pos).cmp(&(self.heuristic, other.node.pos))
    }
}

fn heur(s: &State, t: &State) -> u8 {
    s.manhattan(t).iter().sum()
}

#[allow(dead_code)]
fn old_heur(s: &State, t: &State) -> u8 {
    let a = (s.manhattan(t), s.inversion(t));
    std::cmp::max(a.0[0], a.1[0]) + std::cmp::max(a.0[1], a.1[1])
}

fn astar(s: &State, t: &State) -> u8 {
    use std::collections::{BinaryHeap, HashMap};
    let mut tosee = BinaryHeap::new();
    let mut dists: HashMap<u64, u8, Builder> = HashMap::with_hasher(Builder::default());
    tosee.push(NodeInfo {
        heuristic: heur(s, t),
        node: *s,
    });
    //actually it is enough to store first 56 bits
    dists.insert(s.pos, 1);
    let mut count = 0;
    while let Some(node) = tosee.pop() {
        let pathlength = *dists.get(&node.node.pos).unwrap();
        if pathlength == 0 {
            continue;
        }
        count += 1;
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
    let b = astar(&a, &State::default());
    println!("{}", b);
}
