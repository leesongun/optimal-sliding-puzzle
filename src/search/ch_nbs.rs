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

pub fn ch_nbs(s: &State, t: &State, h: &impl Fn(&State, &State) -> u8) -> u8 {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::collections::VecDeque;

    use array_init::array_init;

    let mut list: [[[VecDeque<State>; 81]; 81]; 2] =
        array_init(|_| array_init(|_| array_init(|_| VecDeque::<State>::new())));

    list[0][0][h(s, t) as usize].push_back(*s);
    list[1][0][h(s, t) as usize].push_back(*t);

    //actually it is enough to store first 56 bits
    let mut dists = [
        HashMap::with_hasher(Builder::default()),
        HashMap::with_hasher(Builder::default()),
    ];

    dists[0].insert(s.pos, 1);
    dists[1].insert(t.pos, 1);

    let mut count = 0;
    let mut UB = 100;
    let mut LB = h(s, t);
    while LB < UB {
        for i in 0..81 {
            for j in 0..81 {
                while let Some(x) = list[0][i][j].front() {
                    if dists[0].get(&x.pos) == Some(&0) {
                        list[0][i][j].pop_front();
                    }
                }
                while let Some(x) = list[1][i][j].front() {
                    if dists[1].get(&x.pos) == Some(&0) {
                        list[1][i][j].pop_front();
                    }
                }
            }
        }

        let mut searchnode: Option<(State, State)> = None;
        'largeloop: for i in 0..81 {
            for j in 0..81 {
                if list[0][i][j].is_empty() {
                    continue;
                }
                for ii in 0..(LB as usize + 1 - j) {
                    for jj in 0..(LB as usize + 1 - i) {
                        if list[1][ii][jj].is_empty() {
                            continue;
                        }
                        searchnode = Some((
                            list[0][i][j].pop_front().unwrap(),
                            list[1][ii][jj].pop_front().unwrap(),
                        ));
                        break 'largeloop;
                    }
                }
                break;
            }
        }
        if let None = searchnode {
            LB += 1; //+=2
            continue;
        }
        let searchnode = searchnode.unwrap();

        count += 1;
        let path = dists[0].insert(searchnode.0.pos, 0).unwrap();

        for &i in &Action::VALUES {
            if let Some(x) = searchnode.0 + i {
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
                    //y : real length + 1
                    //path : real length
                    UB = std::cmp::min(UB, y + path - 1);
                }
                list[0][(path - h(s, &x)) as usize][(path + h(t, &x)) as usize].push_back(x);
            }
        }

        count += 1;
        let path = dists[1].insert(searchnode.1.pos, 0).unwrap();

        for &i in &Action::VALUES {
            if let Some(x) = searchnode.1 + i {
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
                    //y : real length + 1
                    //path : real length
                    UB = std::cmp::min(UB, y + path - 1);
                }
                list[1][(path - h(t, &x)) as usize][(path + h(s, &x)) as usize].push_back(x);
            }
        }
    }
    println!("{}", count);
    UB
}
