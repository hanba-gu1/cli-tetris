use crossterm::{
    cursor::MoveTo,
    execute,
    style::{ResetColor, SetBackgroundColor},
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
            SetBackgroundColor(mino.color())
        )?;
        print!("　");
        execute!(stdout(), ResetColor)?;
    }
    println!();

    Ok(())
}

pub fn display_hold(column: u16, row: u16, held_mino: MinoType) -> Result<()> {
    let height = 4;

    execute!(stdout(), MoveTo(column, row))?;
    print!("┌─── HOLD ───┐");
    for i in 0..height {
        execute!(stdout(), MoveTo(column, row + i + 1))?;
        print!("│            │");
    }
    execute!(stdout(), MoveTo(column, row + height + 1))?;
    println!("└────────────┘");

    display_mino(column + 2, row + 1, held_mino)?;

    Ok(())
}

pub fn display_next(column: u16, row: u16, minos: Vec<MinoType>) -> Result<()> {
    let count = minos.iter().count();
    let height = 3;

    execute!(stdout(), MoveTo(column, row))?;
    print!("┌─── NEXT ───┐");
    for i in 0..count * height + 1 {
        execute!(stdout(), MoveTo(column, row + i as u16 + 1))?;
        print!("│            │");
    }
    execute!(stdout(), MoveTo(column, row + (count * height) as u16 + 2))?;
    println!("└────────────┘");

    for (i, mino) in minos.iter().enumerate() {
        display_mino(column + 2, row + (i * height) as u16 + 1, *mino)?;
    }

    Ok(())
}
