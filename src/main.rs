use std::io::{stdout, Result};
use crossterm::{
    cursor::{Hide, MoveTo}, event::{Event, EventStream, KeyCode, KeyEvent}, execute, style::{Color, ResetColor, SetBackgroundColor}, terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};
use futures::{FutureExt, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;

    let mut reader = EventStream::new();
    loop {
        let input = reader.next().fuse().await;
        if let Some(Ok(event)) = input {
            match event {
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => break,
                Event::Key(KeyEvent { code: KeyCode::Char('r'), .. }) | Event::Resize(_, _) => tokio::task::spawn_blocking(|| display_field(0, 0)).await??,
                _ => {}
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

fn display_field(column: u16, row: u16) -> Result<()> {
    let height = 20;
    let width = 10;
    let edge_color = Color::DarkBlue;
    let field_color = Color::DarkGrey;
    
    execute!(stdout(), MoveTo(column, row), SetBackgroundColor(edge_color))?;
    print!("{}", "　".repeat(width + 2));
    for i in 0..height {
        execute!(stdout(), MoveTo(column, row + i + 1),  SetBackgroundColor(edge_color))?;
        print!("　");
        execute!(stdout(), SetBackgroundColor(field_color))?;
        print!("{}", "・".repeat(width));
        execute!(stdout(), SetBackgroundColor(edge_color))?;
        print!("　");
        execute!(stdout(), ResetColor)?;
        println!();
    }
    execute!(stdout(), MoveTo(column, row + height + 1), SetBackgroundColor(edge_color))?;
    print!("{}", "　".repeat(width + 2));
    execute!(stdout(), MoveTo(column, row), ResetColor)?;

    Ok(())
}
