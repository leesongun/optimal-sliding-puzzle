use astar::{Action, State};
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

fn astar(s: &State, t: &State) -> u8 {
    use std::collections::{BinaryHeap, HashMap};
    let mut tosee = BinaryHeap::new();
    let mut dists: HashMap<State, u8> = HashMap::new();
    tosee.push(NodeInfo {
        heuristic: s.manhattan(t) as u8,
        node: *s,
    });
    dists.insert(*s, 1);
    let mut count = 0;
    while let Some(node) = tosee.pop() {
        if *dists.get(&node.node).unwrap() == 0 {
            continue;
        }
        count += 1;
        if count % 1000000 == 0 {
            println!("{}", count);
        }
        let pathlength = *dists.get(&node.node).unwrap();
        if node.node == *t {
            return pathlength - 1;
        }
        for i in &Action::VALUES {
            let mut x = node.node;
            if x.act(*i) {
                let d = t.manhattan(&x) as u8;
                if let Some(&prev) = dists.get(&x) {
                    if prev <= pathlength + 1 {
                        continue;
                    }
                }
                dists.insert(x, pathlength + 1);
                tosee.push(NodeInfo {
                    heuristic: pathlength + 1 + d,
                    node: x,
                });
            }
        }
        dists.insert(node.node, 0);
    }
    255
}
fn main() {
    use rand::SeedableRng;
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let a = State::rand(&mut rng);
    println!("{}", astar(&a, &State::default()));
}
