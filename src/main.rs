use crossterm::{
    cursor::Hide,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use display::Displayer;
use falling_clock::FallingClock;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{
    collections::VecDeque,
    io::{stdout, Result},
    sync::{Arc, Mutex},
    time::Duration,
};

mod display;
mod event;
mod falling_clock;
mod field;
mod mino;
mod mino_operation;
mod term_operation;

use crate::{
    event::{mino_operation::MinoOperation, Event},
    field::Field,
    mino::{Mino, MinoType},
};

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut rng = rand::rng();
    let game_state = Arc::new(Mutex::new(GameState::new(&mut rng)));
    let displayer = Displayer::new(Arc::clone(&game_state))?;

    main_loop(&mut rng, Arc::clone(&game_state), &displayer).await;

    displayer.exit().await?;

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

async fn main_loop(rng: &mut ThreadRng, game_state: Arc<Mutex<GameState>>, displayer: &Displayer) {
    let mut event_manager = event::EventManager::new();
    let mut falling_clock = FallingClock::new(event_manager.sender());
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
                Event::MinoOperation(event) => {
                    mino_operation::mino_operation(
                        rng,
                        &mut game_state.lock().unwrap(),
                        &mut falling_clock,
                        event,
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
    soft_drop: bool,
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
        let held_mino = None;
        let falling_speed = Duration::from_secs(1);
        let soft_drop = false;
        let can_hold = true;

        Self {
            field,
            current_mino,
            held_mino,
            next_minos,
            falling_speed,
            soft_drop,
            can_hold,
        }
    }
}
