use crossterm::{
    cursor::Hide,
    event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{
    collections::VecDeque,
    io::{stdout, Result},
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

mod field;
use field::Field;

mod slot;

mod mino;
use mino::{Mino, MinoType};

mod operation;
use operation::{change_mino, hold_mino};

mod display;

mod log;

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rand::rng();

    let mut logger = log::Logger::new(Path::new("./debug.log")).await?;

    let game_state = Arc::new(Mutex::new(GameState::new(&mut rng)));

    let falling_speed = Duration::from_millis(1000);

    change_mino(&mut rng, &mut game_state.lock().unwrap());

    let mut timer = Timer::new();

    let displayer = display::Displayer::new(Arc::clone(&game_state))?;

    timer.start(Duration::from_secs(2));

    let mut terminal_event_reader = EventStream::new();
    displayer.display();
    loop {
        let term_event = terminal_event_reader.next().fuse();
        
        tokio::select! {
            Some(Ok(term_event)) = term_event => match term_event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Right => {
                        let game_state = &mut game_state.lock().unwrap();
                        if let Some(current_mino) = &mut game_state.current_mino {
                            current_mino.column += 1;
                            displayer.display();
                        }
                    }
                    KeyCode::Left => {
                        if let Some(current_mino) = &mut game_state.lock().unwrap().current_mino {
                            current_mino.column -= 1;
                            displayer.display();
                        }
                    }
                    KeyCode::Char('c') => {
                        let game_state_temp = &mut game_state.lock().unwrap();
                        if let Some(_) = game_state_temp.current_mino {
                            hold_mino(&mut rng, game_state_temp);
                        }
                        displayer.display();
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {
                    displayer.display();
                }
                _ => {}
            },
            Some(_) = timer.receive() => {
                if let Some(current_mino) = &mut game_state.lock().unwrap().current_mino {
                    current_mino.row += 1;
                }
                displayer.display();
                timer.start(falling_speed);
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    displayer.end();

    Ok(())
}

struct GameState {
    field: Field,
    current_mino: Option<Mino>,
    held_mino: Option<MinoType>,
    next_minos: VecDeque<MinoType>,
}
impl GameState {
    fn new(rng: &mut ThreadRng) -> Self {
        let all_minos = MinoType::all_minos();

        let field = Field::new();
        let next_minos: VecDeque<_> = {
            let mut all_minos = all_minos.clone();
            all_minos.shuffle(rng);
            all_minos.into_iter().collect()
        };
        let current_mino = None;
        let held_mino: Option<MinoType> = None;

        Self {
            field,
            current_mino,
            held_mino,
            next_minos,
        }
    }
}

struct Timer {
    sender: tokio::sync::mpsc::Sender<()>,
    receiver: tokio::sync::mpsc::Receiver<()>,
}
impl Timer {
    fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(128);
        Self { sender, receiver }
    }
    fn start(&self, time: Duration) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(time).await;
            let _ = sender.send(()).await;
        });
    }
    async fn receive(&mut self) -> Option<()> {
        self.receiver.recv().await
    }
}

fn key_pressed(key_event: KeyEvent) {
    // if 
}
