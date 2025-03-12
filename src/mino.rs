use crossterm::style::Color;

use crate::{
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    GameState,
};

#[derive(Clone, Copy)]
pub enum MinoType {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}
impl MinoType {
    pub fn all_minos() -> Vec<MinoType> {
        vec![
            MinoType::I,
            MinoType::J,
            MinoType::L,
            MinoType::O,
            MinoType::S,
            MinoType::T,
            MinoType::Z,
        ]
    }

    pub fn color(&self) -> Color {
        use MinoType::*;
        match self {
            I => Color::Cyan,
            O => Color::Yellow,
            S => Color::Green,
            Z => Color::Red,
            J => Color::Blue,
            L => Color::DarkYellow,
            T => Color::DarkMagenta,
        }
    }
    pub fn blocks(&self, rotation: Rotation) -> &[(u16, u16)] {
        use MinoType::*;
        match &self {
            I => match rotation {
                Rotation::A => &[(1, 0), (1, 1), (1, 2), (1, 3)],
                Rotation::B => &[(0, 2), (1, 2), (2, 2), (3, 2)],
                Rotation::C => &[(2, 0), (2, 1), (2, 2), (2, 3)],
                Rotation::D => &[(0, 1), (1, 1), (2, 1), (3, 1)],
            },
            O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            S => match rotation {
                Rotation::A => &[(0, 1), (0, 2), (1, 0), (1, 1)],
                Rotation::B => &[(0, 1), (1, 1), (1, 2), (2, 2)],
                Rotation::C => &[(1, 1), (1, 2), (2, 0), (2, 1)],
                Rotation::D => &[(0, 0), (1, 0), (1, 1), (2, 1)],
            },
            Z => match rotation {
                Rotation::A => &[(0, 0), (0, 1), (1, 1), (1, 2)],
                Rotation::B => &[(0, 2), (1, 1), (1, 2), (2, 1)],
                Rotation::C => &[(1, 0), (1, 1), (2, 1), (2, 2)],
                Rotation::D => &[(0, 1), (1, 0), (1, 1), (2, 0)],
            },
            J => match rotation {
                Rotation::A => &[(0, 0), (1, 0), (1, 1), (1, 2)],
                Rotation::B => &[(0, 1), (0, 2), (1, 1), (2, 1)],
                Rotation::C => &[(1, 0), (1, 1), (1, 2), (2, 2)],
                Rotation::D => &[(0, 1), (1, 1), (2, 0), (2, 1)],
            },
            L => match rotation {
                Rotation::A => &[(0, 2), (1, 0), (1, 1), (1, 2)],
                Rotation::B => &[(0, 1), (1, 1), (2, 1), (2, 2)],
                Rotation::C => &[(1, 0), (1, 1), (1, 2), (2, 0)],
                Rotation::D => &[(0, 0), (0, 1), (1, 1), (2, 1)],
            },
            T => match rotation {
                Rotation::A => &[(0, 1), (1, 0), (1, 1), (1, 2)],
                Rotation::B => &[(0, 1), (1, 1), (1, 2), (2, 1)],
                Rotation::C => &[(1, 0), (1, 1), (1, 2), (2, 1)],
                Rotation::D => &[(0, 1), (1, 0), (1, 1), (2, 1)],
            },
        }
    }
    fn start_pos(&self) -> (i16, i16) {
        use MinoType::*;
        match self {
            I => (-1, 3),
            O => (0, 4),
            _ => (0, 3),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Rotation {
    A,
    B,
    C,
    D,
}
impl Rotation {
    pub fn rotate_right(&mut self) {
        use Rotation::*;
        *self = match self {
            A => B,
            B => C,
            C => D,
            D => A,
        };
    }
    pub fn rotate_left(&mut self) {
        use Rotation::*;
        *self = match self {
            A => D,
            B => A,
            C => B,
            D => C,
        };
    }
}

#[derive(Clone)]
pub struct Mino {
    pub mino_type: MinoType,
    pub row: i16,
    pub column: i16,
    pub rotation: Rotation,
}
impl Mino {
    pub fn new(mino_type: MinoType) -> Self {
        let (row, column) = mino_type.start_pos();
        Self {
            mino_type,
            row,
            column,
            rotation: Rotation::A,
        }
    }
    pub fn blocks(&self) -> Vec<(i16, i16)> {
        self.mino_type
            .blocks(self.rotation)
            .iter()
            .map(|(r, c)| (self.row + *r as i16, self.column + *c as i16))
            .collect()
    }
}
