use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Colors, ResetColor, SetBackgroundColor, SetColors},
};
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
};

use crate::{FIELD_HEIGHT, FIELD_WIDTH};

pub(super) struct Field {
    blocks: [[Option<Color>; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
}
impl Field {
    pub(super) fn new() -> Self {
        let blocks = [[None; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
        Self { blocks }
    }
}

pub(super) fn display_field(column: u16, row: u16, field: Arc<Mutex<Field>>) -> Result<()> {
    let edge_color1 = Color::DarkGrey;
    let edge_color2 = Color::Grey;
    let field_color = Color::Black;
    let dot_color = Color::DarkGrey;

    let field = field.lock().unwrap();

    execute!(stdout(), MoveTo(column, row))?;
    for i in 0..FIELD_WIDTH + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(if i % 2 == 0 { edge_color1 } else { edge_color2 })
        )?;
        print!("　");
    }
    for (i, blocks_row) in field.blocks.iter().enumerate() {
        execute!(
            stdout(),
            MoveTo(column, row + i as u16 + 1),
            SetBackgroundColor(if i % 2 == 1 { edge_color1 } else { edge_color2 })
        )?;
        print!("　");
        for block in blocks_row {
            execute!(
                stdout(),
                SetColors(Colors::new(dot_color, block.unwrap_or(field_color)))
            )?;
            print!("・");
        }
        execute!(
            stdout(),
            SetBackgroundColor(if (i + FIELD_WIDTH as usize) % 2 == 0 {
                edge_color1
            } else {
                edge_color2
            })
        )?;
        print!("　");
        execute!(stdout(), ResetColor)?;
        println!();
    }
    execute!(stdout(), MoveTo(column, row + FIELD_HEIGHT + 1))?;
    for i in 0..FIELD_WIDTH + 2 {
        execute!(
            stdout(),
            SetBackgroundColor(if (i + FIELD_HEIGHT) % 2 == 1 {
                edge_color1
            } else {
                edge_color2
            })
        )?;
        print!("　");
    }
    execute!(stdout(), MoveTo(column, row), ResetColor)?;

    Ok(())
}
