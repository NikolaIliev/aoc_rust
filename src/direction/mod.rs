pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    pub fn all() -> &'static [Direction] {
        &[
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ]
    }
}
