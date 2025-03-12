use crossterm::{
    cursor::Hide,
    event::{Event as TermEvent, EventStream, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use display::Displayer;
use futures::{FutureExt, StreamExt};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{
    collections::VecDeque,
    io::{stdout, Result},
    sync::{Arc, Mutex},
    time::Duration,
};

mod display;
mod event;
mod field;
mod log;
mod mino;
mod operation;
mod slot;

use event::{Event, EventSender};
use field::Field;
use mino::{Mino, MinoType};
use operation::{change_mino, hold_mino};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rand::rng();
    // let mut logger = log::Logger::new(Path::new("./debug.log")).await?;
    let mut event_manager = event::EventManager::new();
    let game_state = Arc::new(Mutex::new(GameState::new(&mut rng)));
    let falling_speed = Duration::from_millis(1000);
    change_mino(&mut rng, &mut game_state.lock().unwrap());
    let mut timer = Timer::new();
    let displayer = Displayer::new(Arc::clone(&game_state))?;
    timer.start(Duration::from_secs(2));

    let mut terminal_event_reader = EventStream::new();
    displayer.display();
    loop {
        let term_event = terminal_event_reader.next().fuse();

        tokio::select! {
            Some(event) = event_manager.recv() => match event {
                Event::End => break,
            },
            Some(Ok(term_event)) = term_event => match term_event {
                TermEvent::Key(key_event) => {
                    key_pressed(&mut rng, &mut game_state.lock().unwrap(), &displayer, event_manager.sender(), key_event).await;
                },
                TermEvent::Resize(_, _) => {
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

async fn key_pressed(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    displayer: &Displayer,
    event_sender: EventSender,
    key_event: KeyEvent,
) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc => {
                event_sender.send(Event::End).await;
            }
            KeyCode::Right => move_mino(game_state, displayer, 1),
            KeyCode::Left => move_mino(game_state, displayer, -1),
            KeyCode::Char(c) if ['x', 'z'].contains(&c) => rotate_mino(game_state, displayer, c),
            KeyCode::Char('c') => {
                hold_mino(rng, game_state);
                displayer.display();
            }
            _ => {}
        }
    }

    fn move_mino(game_state: &mut GameState, displayer: &Displayer, move_column: i16) {
        if let Some(current_mino) = &mut game_state.current_mino {
            current_mino.column += move_column;
            displayer.display();
        }
    }
    fn rotate_mino(game_state: &mut GameState, displayer: &Displayer, c: char) {
        if let Some(current_mino) = &mut game_state.current_mino {
            match c {
                'x' => current_mino.rotation.rotate_right(),
                'z' => current_mino.rotation.rotate_left(),
                _ => {}
            }
            displayer.display();
        }
    }
}
