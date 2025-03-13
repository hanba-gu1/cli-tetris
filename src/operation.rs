use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    mino::{Mino, MinoType},
    GameState, Timer,
};

pub fn change_mino(rng: &mut ThreadRng, game_state: &mut GameState) {
    let all_minos = MinoType::all_minos();

    if game_state.next_minos.iter().count() <= all_minos.iter().count() {
        let mut shuffle_minos: Vec<_> = all_minos.into_iter().collect();
        shuffle_minos.shuffle(rng);
        game_state.next_minos.extend(shuffle_minos.into_iter());
    }
    game_state.current_mino = Some(Mino::new(game_state.next_minos.pop_front().unwrap()));
}

pub fn hold_mino(rng: &mut ThreadRng, game_state: &mut GameState, falling_timer: &mut Timer) {
    let current_mino = match &mut game_state.current_mino {
        Some(current_mino) => current_mino,
        None => return,
    };

    match &mut game_state.held_mino {
        Some(held_mino) => {
            (*held_mino, *current_mino) = (current_mino.mino_type, Mino::new(*held_mino));
        }
        None => {
            game_state.held_mino = Some(game_state.current_mino.as_ref().unwrap().mino_type);
            change_mino(rng, game_state);
        }
    }
    falling_timer.start(game_state.falling_speed);
}

pub fn fall_mino(rng: &mut ThreadRng, game_state: &mut GameState) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let temp_mino = Mino {
            row: current_mino.row + 1,
            ..*current_mino
        };
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
        } else {
            game_state.field.place_mino(current_mino);
            change_mino(rng, game_state);
        }
    }
}
