use colored::Color;

pub struct Player {
    color: Color,
    points: u64,
}

impl Player {
    pub fn new(is_white: bool) -> Self {
        let color = if is_white { Color::White } else { Color::Black };
        Player { color, points: 0 }
    }
}
