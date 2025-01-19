#![forbid(unsafe_code)]

pub mod r#trait;
use r#trait::{FairRound, Round, UnfairRound};
pub mod config;
use config::{get_game, GameConfig};
mod games;

type Game = Box<dyn Round>;

pub fn play_game(x: &mut Game, fair_rounds: usize, unfair_rounds: usize) -> Option<u8> {
    let mut winner = None;

    for _ in 0..fair_rounds {
        winner = Some(FairRound::play(&mut **x));
    }

    for _ in 0..unfair_rounds {
        winner = Some(UnfairRound::play(&mut **x));
    }

    winner
}

pub fn play_games(game_configs: &Vec<(String, usize, usize)>) -> Vec<Option<u8>> {
    let mut result: Vec<Option<u8>> = Vec::with_capacity(game_configs.len());

    for game_config in game_configs {
        let (serialized_config, fair_rounds, unfair_rounds) = game_config;

        let config = serde_json::from_str::<GameConfig>(serialized_config).unwrap();
        let mut game: Game = get_game(config);
        result.push(play_game(&mut game, *fair_rounds, *unfair_rounds));
    }

    result
}
