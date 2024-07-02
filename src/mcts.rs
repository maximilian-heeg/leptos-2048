use crate::game::{Actions, Game};

use rand::seq::IteratorRandom;

const DEPTH: usize = 20;
const SEARCHES_PER_MOVE: usize = 200;

fn do_move(game: &mut Game, action: Actions) -> bool {
    match action {
        Actions::Left => game.move_left(),
        Actions::Right => game.move_right(),
        Actions::Up => game.move_up(),
        Actions::Down => game.move_down(),
    }
}

pub fn simlulation(game: &Game) -> Option<Actions> {
    let valid_actions = game.valid_moves();

    let mut scores = vec![];

    for action in valid_actions.iter() {
        let mut current_game = *game;

        let _changed = do_move(&mut current_game, *action);

        let mut score = current_game.score;

        for _j in 0..SEARCHES_PER_MOVE {
            let mut level = 1;
            let mut search_game = current_game;
            search_game.add_tile();
            while !search_game.is_game_over() && level < DEPTH {
                search_game.step(random_move(&search_game));
                level += 1;
            }
            score += search_game.score;
        }
        scores.push(score)
    }
    if let Some((_, action)) = scores.iter().zip(valid_actions).max_by_key(|&(val, _)| val) {
        Some(action)
    } else {
        None
    }
}

fn random_move(game: &Game) -> Actions {
    let mut rng = rand::thread_rng();
    let selected = game.valid_moves().into_iter().choose(&mut rng);
    selected.unwrap()
}
