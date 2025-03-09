use crossterm::{
    cursor::Hide,
    event::{Event, EventStream, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use mino::MinoType;
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
};

mod field;
use field::Field;

mod slot;
mod mino;

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let field = Arc::new(Mutex::new(Field::new()));

    let mut reader = EventStream::new();
    loop {
        let input = reader.next().fuse().await;
        if let Some(Ok(event)) = input {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    ..
                })
                | Event::Resize(_, _) => {
                    let field = Arc::clone(&field);
                    tokio::task::spawn_blocking(|| {
                        slot::display_hold(0, 0, MinoType::T)?;
                        field::display_field(15, 0, field)?;
                        slot::display_next(40, 0, vec![MinoType::T; 6])
                    }).await??;
                }
                _ => {}
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
