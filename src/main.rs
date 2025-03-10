use crossterm::{
    cursor::Hide,
    event::{Event, EventStream, KeyCode, KeyEvent},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use futures::{FutureExt, StreamExt};
use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use std::{
    io::{stdout, Result},
    sync::{Arc, Mutex},
};

mod field;
use field::Field;

mod slot;

mod mino;
use mino::{Mino, MinoType};

mod operation;
use operation::{change_mino, hold_mino};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rng();

    let all_minos = MinoType::all_minos();

    let field = Arc::new(Mutex::new(Field::new()));
    let mut next_minos: Vec<_> = all_minos.into_iter().collect();
    next_minos.shuffle(&mut rng);
    let mut current_mino = None;
    let mut held_mino: Option<MinoType> = None;

    change_mino(&mut rng, &mut current_mino, &mut next_minos);

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
                    let current_mino = current_mino.clone();
                    execute!(stdout(), Clear(ClearType::All))?;
                    tokio::task::spawn_blocking(|| {
                        slot::display_hold(0, 0, MinoType::T)?;
                        field::display_field(15, 0, field, current_mino)?;
                        slot::display_next(40, 0, vec![MinoType::T; 6])
                    })
                    .await??;
                }
                _ => {}
            }
        }
    }
    
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
