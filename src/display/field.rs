use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Colors, Print, ResetColor, SetBackgroundColor, SetColors},
};
use std::io::{stdout, Result};

use crate::{
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    mino::Mino,
    GameState,
};

pub fn display_field(column: u16, row: u16, game_state: &GameState) -> Result<()> {
    let edge_color1 = Color::DarkGrey;
    let edge_color2 = Color::Grey;
    let field_color = Color::Black;
    let dot_color = Color::DarkGrey;

    execute!(stdout(), MoveTo(column, row))?;
    for i in 0..FIELD_WIDTH + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(if i % 2 == 0 { edge_color1 } else { edge_color2 }),
            Print("　"),
        )?;
    }
    for (i, blocks_row) in game_state.field.blocks.iter().enumerate() {
        execute!(
            stdout(),
            MoveTo(column, row + i as u16 + 1),
            SetBackgroundColor(if i % 2 == 1 { edge_color1 } else { edge_color2 }),
            Print("　"),
        )?;
        for block in blocks_row {
            execute!(
                stdout(),
                SetColors(Colors::new(dot_color, block.unwrap_or(field_color))),
                Print("　"),
            )?;
        }
        execute!(
            stdout(),
            SetBackgroundColor(if (i + FIELD_WIDTH as usize) % 2 == 0 {
                edge_color1
            } else {
                edge_color2
            }),
            Print("　"),
        )?;
        execute!(stdout(), ResetColor)?;
    }
    execute!(stdout(), MoveTo(column, row + FIELD_HEIGHT + 1))?;
    for i in 0..FIELD_WIDTH + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(if (i + FIELD_HEIGHT) % 2 == 1 {
                edge_color1
            } else {
                edge_color2
            }),
            Print("　"),
        )?;
    }
    execute!(stdout(), ResetColor)?;

    if let Some(current_mino) = &game_state.current_mino {
        let ghost_mino = game_state.field.ghost_mino(current_mino);
        display_mino(column, row, &ghost_mino, true)?;
        display_mino(column, row, current_mino, false)?;
    }

    Ok(())
}

fn display_mino(column: u16, row: u16, mino: &Mino, is_ghost: bool) -> Result<()> {
    let ghost_color = Color::DarkGrey;

    for (r, c) in mino.blocks() {
        let mass_row = row as i16 + 1 + r;
        let mass_column = column as i16 + 2 + c * 2;
        if 0 <= mass_column && 0 <= mass_row {
            let color = if is_ghost {
                ghost_color
            } else {
                mino.mino_type.color()
            };
            execute!(
                stdout(),
                MoveTo(mass_column as u16, mass_row as u16),
                SetBackgroundColor(color),
                Print("　"),
            )?;
            execute!(stdout(), ResetColor)?;
        }
    }

    Ok(())
}
