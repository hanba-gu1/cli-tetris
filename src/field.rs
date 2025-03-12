use crossterm::style::Color;

pub const FIELD_HEIGHT: u16 = 20;
pub const FIELD_WIDTH: u16 = 10;

pub struct Field {
    pub blocks: [[Option<Color>; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
}
impl Field {
    pub(super) fn new() -> Self {
        let blocks = [[None; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
        Self { blocks }
    }
}
