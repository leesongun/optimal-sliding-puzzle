use astar::State;
fn heur(s: &State, t: &State) -> u8 {
    s.manhattan(t).iter().sum()
}

#[allow(dead_code)]
fn old_heur(s: &State, t: &State) -> u8 {
    let a = (s.manhattan(t), s.inversion(t));
    std::cmp::max(a.0[0], a.1[0]) + std::cmp::max(a.0[1], a.1[1])
}

fn test(i: usize) -> bool {
    let a = State::new(astar::INSTANCES[i]);
    let b = State::default();
    astar::search::dibbs(&a, &b, &heur) == astar::ACTUAL[i]
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
