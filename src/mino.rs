use crossterm::style::Color;

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
}

#[derive(Clone, Copy)]
pub enum Rotation {
    A,
    B,
    C,
    D,
}
impl Rotation {
    fn rotate_right(&self) -> Self {
        use Rotation::*;
        match self {
            A => B,
            B => C,
            C => D,
            D => A,
        }
    }
    fn rotate_left(&self) -> Self {
        use Rotation::*;
        match self {
            A => D,
            B => A,
            C => B,
            D => C,
        }
    }
}

pub struct Mino {
    mino_type: MinoType,
    row: u16,
    column: u16,
    rotation: Rotation,
}
