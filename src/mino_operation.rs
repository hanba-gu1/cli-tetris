use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    display::Displayer,
    mino::{Mino, MinoType},
    GameState, Timer,
};

pub fn change_mino(rng: &mut ThreadRng, game_state: &mut GameState, displayer: &Displayer) {
    let all_minos = MinoType::all_minos();

    if game_state.next_minos.iter().count() <= all_minos.iter().count() {
        let mut shuffle_minos: Vec<_> = all_minos.into_iter().collect();
        shuffle_minos.shuffle(rng);
        game_state.next_minos.extend(shuffle_minos.into_iter());
    }
    game_state.current_mino = Some(Mino::new(game_state.next_minos.pop_front().unwrap()));
    displayer.next();
    displayer.field();
}

pub fn hold_mino(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    displayer: &Displayer,
    falling_timer: &mut Timer,
) {
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
            change_mino(rng, game_state, displayer);
        }
    }
    displayer.hold();
    displayer.field();
    falling_timer.start(game_state.falling_speed);
}

pub fn fall_mino(rng: &mut ThreadRng, game_state: &mut GameState, displayer: &Displayer) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let temp_mino = Mino {
            row: current_mino.row + 1,
            ..*current_mino
        };
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
        } else {
            game_state.field.place_mino(current_mino);
            change_mino(rng, game_state, displayer);
        }
        displayer.field();
    }
}

pub fn move_mino(
    game_state: &mut GameState,
    displayer: &Displayer,
    move_row: i16,
    move_column: i16,
) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let mut temp_mino = current_mino.clone();
        temp_mino.column += move_column;
        temp_mino.row += move_row;
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
            displayer.field();
        }
    }
}

pub fn rotate_mino(game_state: &mut GameState, displayer: &Displayer, c: char) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let mut temp_mino = current_mino.clone();
        match c {
            'x' => temp_mino.rotation.rotate_right(),
            'z' => temp_mino.rotation.rotate_left(),
            _ => {}
        }
        for (r, c) in temp_mino.super_rotation(current_mino.rotation) {
            let temp_mino = Mino {
                row: temp_mino.row + r,
                column: temp_mino.column + c,
                ..temp_mino
            };
            if game_state.field.can_move(&temp_mino) {
                *current_mino = temp_mino;
                displayer.field();
                break;
            }
        }
    }
}

pub fn hard_drop(rng: &mut ThreadRng, game_state: &mut GameState, displayer: &Displayer) {
    if let Some(current_mino) = &mut game_state.current_mino {
        *current_mino = game_state.field.ghost_mino(current_mino);
        game_state.field.place_mino(current_mino);
        change_mino(rng, game_state, displayer);
    }
}
