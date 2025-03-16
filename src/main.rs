use crossterm::{
    cursor::Hide,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use display::Displayer;
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
mod mino;
mod mino_operation;
mod term_operation;
mod timer;

use event::{mino_operation::MinoOperation, Event};
use field::Field;
use mino::{Mino, MinoType};
use timer::Timer;

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rand::rng();
    let game_state = Arc::new(Mutex::new(GameState::new(&mut rng)));
    let displayer = Displayer::new(Arc::clone(&game_state))?;

    main_loop(&mut rng, Arc::clone(&game_state), &displayer).await;

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

async fn main_loop(rng: &mut ThreadRng, game_state: Arc<Mutex<GameState>>, displayer: &Displayer) {
    let mut event_manager = event::EventManager::new();
    let mut falling_timer = Timer::new(event_manager.sender());
    falling_timer.start(
        game_state.lock().unwrap().falling_speed,
        Event::MinoOperation(MinoOperation::Fall),
    );
    tokio::spawn(term_operation::term_operation(event_manager.sender()));
    event_manager
        .send(Event::MinoOperation(MinoOperation::Change))
        .await;
    displayer.all();

    let mut pre_game_state = game_state.lock().unwrap().clone();

    loop {
        if let Some(event) = event_manager.recv().await {
            match event {
                Event::End => break,
                Event::DisplayAll => displayer.all(),
                Event::MinoOperation(mino_operation) => {
                    mino_operation::mino_operation(
                        rng,
                        &mut game_state.lock().unwrap(),
                        &mut falling_timer,
                        mino_operation,
                    )
                    .await
                }
            }
        }

        let game_state = game_state.lock().unwrap();
        if game_state.field != pre_game_state.field
            || game_state.current_mino != pre_game_state.current_mino
        {
            displayer.field();
        }
        if game_state.held_mino != pre_game_state.held_mino {
            displayer.hold();
        }
        if game_state.next_minos != pre_game_state.next_minos {
            displayer.next();
        }
        pre_game_state = game_state.clone();
    }
}

#[derive(Clone)]
struct GameState {
    field: Field,
    current_mino: Option<Mino>,
    held_mino: Option<MinoType>,
    next_minos: VecDeque<MinoType>,
    falling_speed: Duration,
    can_hold: bool,
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
        let can_hold = true;

        Self {
            field,
            current_mino,
            held_mino,
            next_minos,
            falling_speed,
            can_hold,
        }
    }
}
