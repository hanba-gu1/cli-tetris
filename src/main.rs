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
use tokio::sync::mpsc;

mod display;
mod event;
mod field;
mod mino;
mod mino_operation;

use event::{Event, EventSender};
use field::Field;
use mino::{Mino, MinoType};
use mino_operation::{change_mino, fall_mino, hard_drop, hold_mino, move_mino, rotate_mino};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rand::rng();
    let game_state = Arc::new(Mutex::new(GameState::new(&mut rng)));
    let displayer = Displayer::new(Arc::clone(&game_state))?;
    change_mino(&mut rng, &mut game_state.lock().unwrap(), &displayer);

    main_loop(&mut rng, Arc::clone(&game_state), &displayer).await;

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

async fn main_loop(rng: &mut ThreadRng, game_state: Arc<Mutex<GameState>>, displayer: &Displayer) {
    let mut falling_timer = Timer::new();
    let mut event_manager = event::EventManager::new();
    let mut term_event_reader = EventStream::new();
    falling_timer.start(game_state.lock().unwrap().falling_speed);
    displayer.all();

    loop {
        tokio::select! {
            Some(event) = event_manager.recv() => match event {
                Event::End => break,
            },
            Some(Ok(term_event)) = term_event_reader.next().fuse() => match term_event {
                TermEvent::Key(key_event) => {
                    key_pressed(rng, &mut game_state.lock().unwrap(), displayer, &mut falling_timer, event_manager.sender(), key_event).await;
                },
                TermEvent::Resize(_, _) => {
                    displayer.all();
                }
                _ => {}
            },
            Some(_) = falling_timer.receive() => {
                let game_state = &mut game_state.lock().unwrap();
                fall_mino(rng, game_state, displayer);
                falling_timer.start(game_state.falling_speed);
            }
        }
    }
}

struct GameState {
    field: Field,
    current_mino: Option<Mino>,
    held_mino: Option<MinoType>,
    next_minos: VecDeque<MinoType>,
    falling_speed: Duration,
}
impl GameState {
    fn new(rng: &mut ThreadRng) -> Self {
        let field = Field::new();
        let next_minos: VecDeque<_> = {
            let mut all_minos = MinoType::all_minos();
            all_minos.shuffle(rng);
            all_minos.into_iter().collect()
        };
        let current_mino = None;
        let held_mino: Option<MinoType> = None;
        let falling_speed = Duration::from_secs(1);

        Self {
            field,
            current_mino,
            held_mino,
            next_minos,
            falling_speed,
        }
    }
}

struct Timer {
    sender: mpsc::Sender<()>,
    receiver: mpsc::Receiver<()>,
    handle: Option<tokio::task::JoinHandle<()>>,
}
impl Timer {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(128);
        Self {
            sender,
            receiver,
            handle: None,
        }
    }
    fn start(&mut self, time: Duration) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        let sender = self.sender.clone();
        self.handle = Some(tokio::spawn(async move {
            tokio::time::sleep(time).await;
            let _ = sender.send(()).await;
        }));
    }
    fn stop(&mut self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
    }
    async fn receive(&mut self) -> Option<()> {
        self.receiver.recv().await
    }
}

async fn key_pressed(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    displayer: &Displayer,
    falling_timer: &mut Timer,
    event_sender: EventSender,
    key_event: KeyEvent,
) {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc => event_sender.send(Event::End).await,
            KeyCode::Right => move_mino(game_state, displayer, 0, 1),
            KeyCode::Left => move_mino(game_state, displayer, 0, -1),
            KeyCode::Down => {
                move_mino(game_state, displayer, 1, 0);
                falling_timer.start(game_state.falling_speed);
            }
            KeyCode::Char(c) if ['x', 'z'].contains(&c) => rotate_mino(game_state, displayer, c),
            KeyCode::Char('c') => {
                hold_mino(rng, game_state, displayer, falling_timer);
            }
            KeyCode::Char(' ') => hard_drop(rng, game_state, displayer),
            _ => {}
        }
    }
}
