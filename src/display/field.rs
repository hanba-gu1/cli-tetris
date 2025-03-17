use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
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

    let mut display_blocks = game_state.field.display_blocks();

    if let Some(current_mino) = &game_state.current_mino {
        let ghost_mino = game_state.field.ghost_mino(current_mino);
        display_mino(&ghost_mino, true, &mut display_blocks);
        display_mino(current_mino, false, &mut display_blocks);
    }

    execute!(stdout(), MoveTo(column, row))?;
    for i in 0..FIELD_WIDTH + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(if i % 2 == 0 { edge_color1 } else { edge_color2 }),
            Print("　"),
        )?;
    }
    for (i, blocks_row) in display_blocks.iter().enumerate() {
        execute!(
            stdout(),
            MoveTo(column, row + i as u16 + 1),
            SetBackgroundColor(if i % 2 == 1 { edge_color1 } else { edge_color2 }),
            Print("　"),
        )?;
        for block in blocks_row {
            execute!(
                stdout(),
                SetBackgroundColor(block.unwrap_or(field_color)),
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

    Ok(())
}

fn display_mino(
    mino: &Mino,
    is_ghost: bool,
    display_block: &mut [[Option<Color>; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
) {
    let ghost_color = Color::DarkGrey;

    for (r, c) in mino.blocks() {
        if 0 <= r && r < FIELD_HEIGHT as i16 && 0 <= c && c < FIELD_WIDTH as i16 {
            let color = if is_ghost {
                ghost_color
            } else {
                mino.mino_type.color()
            };
            display_block[r as usize][c as usize] = Some(color);
        }
    }
}
