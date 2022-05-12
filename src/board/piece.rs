#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl From<Color> for bool {
    fn from(color: Color) -> bool {
        match color {
            Color::White => true,
            Color::Black => false,
        }
    }
}
