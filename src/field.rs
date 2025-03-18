use crossterm::style::Color;

use crate::mino::Mino;

pub(super) const FIELD_HEIGHT: u16 = 20;
pub(super) const FIELD_WIDTH: u16 = 10;

#[derive(Clone, PartialEq)]
pub(super) struct Field {
    blocks: [[Option<Color>; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize * 2],
}
impl Field {
    pub(super) fn new() -> Self {
        let blocks = [[None; FIELD_WIDTH as usize]; 2 * FIELD_HEIGHT as usize];
        Self { blocks }
    }
    pub(super) fn display_blocks(
        &self,
    ) -> [[Option<Color>; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize] {
        let mut display_blocks = [[None; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
        for (display_block, block) in display_blocks
            .iter_mut()
            .zip(self.blocks.iter().skip(FIELD_HEIGHT as usize))
        {
            *display_block = block.clone();
        }
        display_blocks
    }
    fn is_empty(&self, row: i16, column: i16) -> bool {
        (-(FIELD_HEIGHT as i16) <= row && row < FIELD_HEIGHT as i16)
            && (0 <= column && column < FIELD_WIDTH as i16)
            && self.blocks[(row + FIELD_HEIGHT as i16) as usize][column as usize].is_none()
    }
    pub(super) fn can_move(&self, mino: &Mino) -> bool {
        mino.blocks().iter().all(|(r, c)| self.is_empty(*r, *c))
    }
    pub(super) fn on_ground(&self, mino: &Mino) -> bool {
        !mino.blocks().iter().all(|(r, c)| self.is_empty(*r + 1, *c))
    }
    pub(super) fn place_mino(&mut self, mino: &Mino) {
        for (r, c) in mino.blocks() {
            self.blocks[(r + FIELD_HEIGHT as i16) as usize][c as usize] =
                Some(mino.mino_type.color());
        }
        self.clear_lines();
    }
    pub(super) fn clear_lines(&mut self) {
        let mut clear_lines_count = 0;
        for row in (0..FIELD_HEIGHT as i16 * 2).rev() {
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
    pub(super) fn ghost_mino(&self, mino: &Mino) -> Mino {
        let mut ghost_mino = mino.clone();
        while !self.on_ground(&ghost_mino) {
            ghost_mino.row += 1;
        }
        ghost_mino
    }
}
