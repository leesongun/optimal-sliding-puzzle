use astar::{Action, Builder, State};
use std::cmp::Ordering;

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

fn heur(s: &State, t: &State) -> u8 {
    s.manhattan(t).iter().sum()
}

#[allow(dead_code)]
fn old_heur(s: &State, t: &State) -> u8 {
    let a = (s.manhattan(t), s.inversion(t));
    std::cmp::max(a.0[0], a.1[0]) + std::cmp::max(a.0[1], a.1[1])
}

#[allow(dead_code)]
fn astar(s: &State, t: &State) -> u8 {
    use std::collections::hash_map::Entry;
    use std::collections::{BinaryHeap, HashMap};

    let mut queue = BinaryHeap::new();
    queue.push(Node(heur(s, t), *s));

    //actually it is enough to store first 56 bits
    let mut dists = HashMap::with_hasher(Builder::default());
    dists.insert(s.pos, 1);

    let mut count = 0;
    while let Some(Node(_, state)) = queue.pop() {
        let pathlength = dists.insert(state.pos, 0).unwrap();
        if pathlength == 0 {
            continue;
        }
        count += 1;
        if state == *t {
            println!("{}", count);
            return pathlength - 1;
        }
        for &i in &Action::VALUES {
            if let Some(x) = state + i {
                let d = heur(t, &x);
                match dists.entry(x.pos) {
                    Entry::Occupied(mut entry) => {
                        if *entry.get() <= pathlength + 1 {
                            continue;
                        }
                        entry.insert(pathlength + 1);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(pathlength + 1);
                    }
                }
                queue.push(Node(pathlength + 1 + d, x));
            }
        }
    }
    255
}

#[allow(dead_code)]
fn idastar(s: &State, t: &State) -> u8 {
    let mut d = heur(s, t);
    loop {
        let x = search(*s, t, 0, d);
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

fn search(s: State, t: &State, p: u64, d: u8) -> u8 {
    if s == *t {
        return 0;
    }
    let h = heur(&s, t);
    if h > d {
        return h;
    }
    let mut min: u8 = 254;
    for i in &Action::VALUES {
        if let Some(x) = s + *i {
            if x.val == p {
                continue;
            }
            let t = search(x, t, s.val, d - 1);
            if t == 0 {
                return 0;
            }
            min = std::cmp::min(min, t);
        }
    }
    min + 1
}

//bidirectional pathmax
#[allow(dead_code)]
fn idastar_bpmx(s: &State, t: &State) -> u8 {
    let mut d = heur(s, t);
    loop {
        let r = search_bpmx(*s, t, 0, d);
        if r == 0 {
            return d;
        } else {
            d = r + 2;
            //theoretically 1, but we know parity in this case
        }
    }
}

//transposition table?
fn search_bpmx(s: State, t: &State, p: u64, d: u8) -> u8 {
    if s == *t {
        return 0;
    }
    let h = heur(&s, t);
    if h > d {
        return h;
    }
    for i in &Action::VALUES {
        if let Some(x) = s + *i {
            if x.val == p {
                continue;
            }
            let t = search_bpmx(x, t, s.val, d - 1);
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

fn test(i: usize) -> bool {
    let a = State::new(astar::INSTANCES[i]);
    let b = State::default();
    idastar(&a, &b) == astar::ACTUAL[i]
}

#[allow(dead_code)]
#[must_use]
fn rand() -> State {
    use rand::SeedableRng;
    let mut rng = rand::rngs::SmallRng::from_entropy();
    State::rand(&mut rng)
}

fn main() {
    for i in 0..100 {
        print!("testing instance {} : ", i);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        if !test(i) {
            println!("failed");
        } else {
            println!("sucess");
        }
    }
}
