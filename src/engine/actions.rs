#[derive(Debug, PartialEq)]
pub enum Direction {West, East, North, South}

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    Movement(Direction),
    Attack(Direction),
    Shout(String)
}