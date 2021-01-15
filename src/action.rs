#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    UP = -4,
    LEFT = -1,
    RIGHT = 1,
    DOWN = 4,
}

impl Action {
    pub const VALUES: [Self; 4] = [Action::UP, Action::LEFT, Action::RIGHT, Action::DOWN];
}
