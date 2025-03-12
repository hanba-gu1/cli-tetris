use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    mino::{Mino, MinoType},
    GameState,
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

pub fn hold_mino(rng: &mut ThreadRng, game_state: &mut GameState) {
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
}
