use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::mino::{Mino, MinoType};

pub fn change_mino(rng: &mut ThreadRng, current_mino: &mut Option<Mino>, next_minos: &mut Vec<MinoType>) {
    let all_minos  = MinoType::all_minos();
    
    if next_minos.iter().count() <= all_minos.iter().count() {
        let mut shuffle_minos: Vec<_> = all_minos.into_iter().collect();
        shuffle_minos.shuffle(rng);
        next_minos.extend(shuffle_minos.into_iter());
    }
    *current_mino = Some(Mino::new(next_minos.pop().unwrap()));
}

pub fn hold_mino(rng: &mut ThreadRng, current_mino: &mut Option<Mino>, held_mino: &mut Option<MinoType>, next_minos: &mut Vec<MinoType>) {
    let current_mino = match current_mino {
        Some(current_mino) => current_mino,
        None => return,
    };
    match held_mino {
        Some(held_mino) => {
            let temp = *held_mino;
            *held_mino = current_mino.mino_type;
            *current_mino = Mino::new(temp);
        }
        None => {

        }
    }
}
