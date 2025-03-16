use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    event::{
        mino_operation::{Direction, MinoOperation},
        Event,
    },
    mino::{Mino, MinoType},
    GameState, Timer,
};

pub async fn mino_operation(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_timer: &mut Timer,
    event: MinoOperation,
) {
    match event {
        MinoOperation::Move(direction) => {
            let (move_row, move_column) = match direction {
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            };
            move_mino(game_state, move_row, move_column);
        }
        MinoOperation::Fall => fall_mino(rng, game_state, falling_timer),
        MinoOperation::RotateLeft => rotate_mino(game_state, 'z'),
        MinoOperation::RotateRight => rotate_mino(game_state, 'x'),
        MinoOperation::Hold => hold_mino(rng, game_state, falling_timer),
        MinoOperation::HardDrop => hard_drop(rng, game_state, falling_timer),
        MinoOperation::SoftDrop => move_mino(game_state, 1, 0),
        MinoOperation::Change => change_mino(rng, game_state, falling_timer),
    }
}

fn change_mino(rng: &mut ThreadRng, game_state: &mut GameState, falling_timer: &mut Timer) {
    let all_minos = MinoType::all_minos();

    if game_state.next_minos.iter().count() <= all_minos.iter().count() {
        let mut shuffle_minos: Vec<_> = all_minos.into_iter().collect();
        shuffle_minos.shuffle(rng);
        game_state.next_minos.extend(shuffle_minos.into_iter());
    }
    game_state.current_mino = Some(Mino::new(game_state.next_minos.pop_front().unwrap()));
    game_state.can_hold = true;
    falling_timer.start(
        game_state.falling_speed,
        Event::MinoOperation(MinoOperation::Fall),
    );
}

fn hold_mino(rng: &mut ThreadRng, game_state: &mut GameState, falling_timer: &mut Timer) {
    let current_mino = match &mut game_state.current_mino {
        Some(current_mino) => current_mino,
        None => return,
    };
    if !game_state.can_hold {
        return;
    }

    match &mut game_state.held_mino {
        Some(held_mino) => {
            (*held_mino, *current_mino) = (current_mino.mino_type, Mino::new(*held_mino));
        }
        None => {
            game_state.held_mino = Some(game_state.current_mino.as_ref().unwrap().mino_type);
            change_mino(rng, game_state, falling_timer);
        }
    }
    game_state.can_hold = false;
    falling_timer.start(
        game_state.falling_speed,
        Event::MinoOperation(MinoOperation::Fall),
    );
}

fn fall_mino(rng: &mut ThreadRng, game_state: &mut GameState, falling_timer: &mut Timer) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let temp_mino = Mino {
            row: current_mino.row + 1,
            ..*current_mino
        };
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
        } else {
            place_mino(game_state);
            change_mino(rng, game_state, falling_timer);
        }
        falling_timer.start(
            game_state.falling_speed,
            Event::MinoOperation(MinoOperation::Fall),
        );
    }
}

fn move_mino(game_state: &mut GameState, move_row: i16, move_column: i16) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let mut temp_mino = current_mino.clone();
        temp_mino.column += move_column;
        temp_mino.row += move_row;
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
        }
    }
}

fn rotate_mino(game_state: &mut GameState, c: char) {
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
                break;
            }
        }
    }
}

fn hard_drop(rng: &mut ThreadRng, game_state: &mut GameState, falling_timer: &mut Timer) {
    if let Some(current_mino) = &mut game_state.current_mino {
        *current_mino = game_state.field.ghost_mino(current_mino);
        place_mino(game_state);
        change_mino(rng, game_state, falling_timer);
    }
}

fn place_mino(game_state: &mut GameState) {
    if let Some(current_mino) = &mut game_state.current_mino {
        game_state.field.place_mino(current_mino);
        game_state.current_mino = None;
    }
}
