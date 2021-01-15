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
