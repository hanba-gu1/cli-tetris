use crossterm::style::Color;

use crate::mino::Mino;

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
    fn is_empty(&self, row: i16, column: i16) -> bool {
        (0 <= row && row < FIELD_HEIGHT as i16)
            && (0 <= column && column < FIELD_WIDTH as i16)
            && self.blocks[row as usize][column as usize].is_none()
    }
    pub fn can_move(&self, mino: &Mino) -> bool {
        mino.blocks().iter().all(|(r, c)| self.is_empty(*r, *c))
    }
}
