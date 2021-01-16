#![cfg(test)]
use super::*;

#[test]
fn default() {
    assert_eq!(State::default(), State::new(0xFEDCBA9876543210));
}

#[test]
fn move_test() {
    use Action::*;
    let mut temp = State::default();
    assert!(!temp.act(LEFT));
    assert!(!temp.act(UP));
    assert!(temp.act(DOWN));
    assert!(temp.act(RIGHT));
    assert!(temp.act(LEFT));
    assert!(temp.act(UP));
    assert_eq!(temp, State::default());
}

const fn getx(s: &State, ind: u64) -> u64 {
    s.pos >> ind * 4 & 3
}
const fn gety(s: &State, ind: u64) -> u64 {
    s.pos >> ind * 4 + 2 & 3
}

fn manhattan(first: &State, second: &State) -> [u8; 2] {
    [
        (1..16)
            .map(|i| (getx(first, i) as i8 - getx(second, i) as i8).abs())
            .sum::<i8>() as u8,
        (1..16)
            .map(|i| (gety(first, i) as i8 - gety(second, i) as i8).abs())
            .sum::<i8>() as u8,
    ]
}

#[test]
fn manhattan_test() {
    let a = State::default();
    for i in 0..100 {
        let b = State::new(INSTNACES[i]);
        assert!(b.verify(), "error at instance {}", i);
        assert_eq!(manhattan(&a, &b), a.manhattan(&b), "{}", i);
        assert_eq!(a.manhattan(&b).iter().sum::<u8>(), ESTIMATE[i], "{}", i);
        assert_eq!(b.manhattan(&a).iter().sum::<u8>(), ESTIMATE[i], "{}", i);
    }
}
