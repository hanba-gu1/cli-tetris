use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetBackgroundColor},
};
use std::io::{stdout, Result};

use crate::mino::{MinoType, Rotation};

fn display_mino(column: u16, row: u16, mino: MinoType) -> Result<()> {
    for (r, c) in mino.blocks(Rotation::A) {
        let move_to = match mino {
            MinoType::I => column + 1 + c * 2,
            MinoType::O => column + 3 + c * 2,
            _ => column + 2 + c * 2,
        };
        execute!(
            stdout(),
            MoveTo(move_to, row + 1 + r),
            SetBackgroundColor(mino.color()),
            Print("　")
        )?;
        execute!(stdout(), ResetColor)?;
    }

    Ok(())
}

pub fn display_hold(column: u16, row: u16, held_mino: &Option<MinoType>) -> Result<()> {
    let height = 4;

    execute!(stdout(), MoveTo(column, row), Print("┌─── HOLD ───┐"))?;
    for i in 0..height {
        execute!(
            stdout(),
            MoveTo(column, row + i + 1),
            Print("│            │")
        )?;
    }
    execute!(
        stdout(),
        MoveTo(column, row + height + 1),
        Print("└────────────┘")
    )?;

    if let Some(held_mino) = held_mino {
        display_mino(column + 2, row + 1, *held_mino)?;
    }

    Ok(())
}

pub fn display_next(column: u16, row: u16, minos: &[MinoType]) -> Result<()> {
    let count = 6;
    let minos: Vec<_> = minos.iter().take(count).map(|m| *m).collect();
    let height = 3;

    execute!(stdout(), MoveTo(column, row), Print("┌─── NEXT ───┐"))?;
    for i in 0..count * height + 1 {
        execute!(
            stdout(),
            MoveTo(column, row + i as u16 + 1),
            Print("│            │")
        )?;
    }
    execute!(
        stdout(),
        MoveTo(column, row + (count * height) as u16 + 2),
        Print("└────────────┘")
    )?;

    for (i, mino) in minos.into_iter().enumerate() {
        display_mino(column + 2, row + (i * height) as u16 + 1, mino)?;
    }

    Ok(())
}
