use crossterm::style::Color;

use crate::mino::Mino;

pub const FIELD_HEIGHT: u16 = 20;
pub const FIELD_WIDTH: u16 = 10;

#[derive(Clone, PartialEq)]
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
    pub fn place_mino(&mut self, mino: &Mino) {
        for (r, c) in mino.blocks() {
            self.blocks[r as usize][c as usize] = Some(mino.mino_type.color());
        }
        self.clear_lines();
    }
    pub fn clear_lines(&mut self) {
        let mut clear_lines_count = 0;
        for row in (0..FIELD_HEIGHT as i16).rev() {
            if self.blocks[row as usize]
                .iter()
                .all(|block| block.is_some())
            {
                clear_lines_count += 1;
            } else {
                if clear_lines_count > 0 {
                    self.blocks[(row + clear_lines_count) as usize] = self.blocks[row as usize];
                    self.blocks[row as usize].fill(None);
                }
            }
        }
    }
    pub fn ghost_mino(&self, mino: &Mino) -> Mino {
        let mut ghost_mino = mino.clone();
        while self.can_move(&ghost_mino) {
            ghost_mino.row += 1;
        }
        ghost_mino.row -= 1;
        ghost_mino
    }
}
