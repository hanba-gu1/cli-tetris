use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    event::mino_operation::{Direction, MinoOperation},
    falling_clock::FallingClock,
    mino::{Mino, MinoType},
    GameState,
};

pub(super) async fn mino_operation(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_clock: &mut FallingClock,
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
        MinoOperation::Fall => fall_mino(rng, game_state, falling_clock).await,
        MinoOperation::RotateLeft => rotate_mino(game_state, false),
        MinoOperation::RotateRight => rotate_mino(game_state, true),
        MinoOperation::Hold => hold_mino(rng, game_state, falling_clock).await,
        MinoOperation::HardDrop => hard_drop(rng, game_state, falling_clock).await,
        MinoOperation::StartSoftDrop => start_soft_drop(game_state, falling_clock).await,
        MinoOperation::EndSoftDrop => end_soft_drop(game_state, falling_clock).await,
        MinoOperation::Change => change_mino(rng, game_state, falling_clock).await,
    }
}

async fn change_mino(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_clock: &mut FallingClock,
) {
    let all_minos = MinoType::all_minos();

    if game_state.next_minos.iter().count() <= all_minos.iter().count() {
        let mut shuffle_minos: Vec<_> = all_minos.into_iter().collect();
        shuffle_minos.shuffle(rng);
        game_state.next_minos.extend(shuffle_minos.into_iter());
    }
    game_state.current_mino = Some(Mino::new(game_state.next_minos.pop_front().unwrap()));
    while !game_state
        .field
        .can_move(game_state.current_mino.as_ref().unwrap())
    {
        game_state.current_mino.as_mut().unwrap().row -= 1;
    }
    game_state.can_hold = true;
    start_falling_clock(game_state, falling_clock).await;
}

async fn hold_mino(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_clock: &mut FallingClock,
) {
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
            change_mino(rng, game_state, falling_clock).await;
        }
    }
    game_state.can_hold = false;
}

async fn fall_mino(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_clock: &mut FallingClock,
) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let temp_mino = Mino {
            row: current_mino.row + 1,
            ..*current_mino
        };
        if game_state.field.can_move(&temp_mino) {
            *current_mino = temp_mino;
        } else {
            place_mino(game_state);
            change_mino(rng, game_state, falling_clock).await;
        }
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

fn rotate_mino(game_state: &mut GameState, is_right: bool) {
    if let Some(current_mino) = &mut game_state.current_mino {
        let mut temp_mino = current_mino.clone();
        if is_right {
            temp_mino.rotation.rotate_right();
        } else {
            temp_mino.rotation.rotate_left();
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

async fn hard_drop(
    rng: &mut ThreadRng,
    game_state: &mut GameState,
    falling_clock: &mut FallingClock,
) {
    if let Some(current_mino) = &mut game_state.current_mino {
        *current_mino = game_state.field.ghost_mino(current_mino);
        place_mino(game_state);
        change_mino(rng, game_state, falling_clock).await;
    }
}

async fn start_soft_drop(game_state: &mut GameState, falling_clock: &mut FallingClock) {
    game_state.soft_drop = true;
    start_falling_clock(game_state, falling_clock).await;
}
async fn end_soft_drop(game_state: &mut GameState, falling_clock: &mut FallingClock) {
    game_state.soft_drop = false;
    start_falling_clock(game_state, falling_clock).await;
}

fn place_mino(game_state: &mut GameState) {
    if let Some(current_mino) = &mut game_state.current_mino {
        game_state.field.place_mino(current_mino);
        game_state.current_mino = None;
    }
}

async fn start_falling_clock(game_state: &mut GameState, falling_clock: &mut FallingClock) {
    let falling_speed = if game_state.soft_drop {
        game_state.falling_speed / 20
    } else {
        game_state.falling_speed
    };
    falling_clock.start(falling_speed).await;
}
