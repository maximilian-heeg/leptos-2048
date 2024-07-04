use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, convert::TryFrom};
use strum::IntoEnumIterator;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

const SIZE: usize = 4;
const MAX_EXPONENT: usize = 17; // log2(131,072) is 17

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tile {
    pub idx: u32,
    pub value: u32,
    new: bool,
    changed: bool,
}

impl Tile {
    pub fn new(idx: u32, value: u32) -> Self {
        Tile {
            idx,
            value,
            new: true,
            changed: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Game {
    pub board: [[Tile; SIZE]; SIZE],
    pub score: u32,
    pub moves: usize,
    pub tiles: u32,
}

#[derive(Debug, Copy, Clone, EnumIter, EnumCountMacro, Serialize, Deserialize)]
pub enum Actions {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<usize> for Actions {
    type Error = ();
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Actions::Left as usize => Ok(Actions::Left),
            x if x == Actions::Right as usize => Ok(Actions::Right),
            x if x == Actions::Up as usize => Ok(Actions::Up),
            x if x == Actions::Down as usize => Ok(Actions::Down),
            _ => Err(()),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            board: [[Tile::new(0, 0); SIZE]; SIZE],
            score: 0,
            moves: 0,
            tiles: 0,
        };
        game.add_tile();
        game.add_tile();
        game
    }

    pub fn highest_tile(&self) -> Option<u32> {
        Some(u32::pow(
            2,
            self.board
                .iter()
                .flat_map(|row| row.iter().map(|tile| tile.value))
                .max()?,
        ))
    }

    /// Return the hashmap of all tiles
    /// index is a unique id, then i, j and value
    pub fn tiles(&self) -> BTreeMap<u32, (usize, usize, u32, bool, bool)> {
        let mut hm: BTreeMap<u32, (usize, usize, u32, bool, bool)> = BTreeMap::new();

        self.board.iter().enumerate().for_each(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &tile)| tile.value > 0)
                .for_each(|(j, tile)| {
                    hm.insert(
                        tile.idx,
                        (i, j, u32::pow(2, tile.value), tile.new, tile.changed),
                    );
                })
        });
        hm
    }

    pub fn valid_moves(&self) -> Vec<Actions> {
        let mut moves: Vec<Actions> = vec![];
        for action in Actions::iter() {
            let mut current_game = *self;

            let changed = match action {
                Actions::Left => current_game.move_left(),
                Actions::Right => current_game.move_right(),
                Actions::Up => current_game.move_up(),
                Actions::Down => current_game.move_down(),
            };

            if changed {
                moves.push(action)
            }
        }
        moves
    }

    pub fn empty_tiles(&self) -> Vec<(usize, usize)> {
        let empty_tiles: Vec<(usize, usize)> = self
            .board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &tile)| tile.value == 0)
                    .map(move |(j, _)| (i, j))
            })
            .collect();
        empty_tiles
    }

    pub fn add_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let empty_tiles = self.empty_tiles();
        self.tiles += 1;

        if let Some(&(i, j)) = empty_tiles.choose(&mut rng) {
            let value = if rng.gen_range(0..10) == 0 { 2 } else { 1 };
            self.board[i][j] = Tile::new(self.tiles, value);
        }
    }

    pub fn step(&mut self, action: Actions) -> bool {
        let changed = match action {
            Actions::Left => self.move_left(),
            Actions::Right => self.move_right(),
            Actions::Up => self.move_up(),
            Actions::Down => self.move_down(),
        };

        if changed {
            self.add_tile();
            self.moves += 1;
        }
        changed
    }

    pub fn move_left(&mut self) -> bool {
        let mut changed = false;
        for row in &mut self.board {
            let mut new_row = [Tile::new(0, 0); SIZE];
            let mut pos = 0;
            for &tile in row.iter().filter(|&&tile| tile.value != 0) {
                if new_row[pos].value == tile.value {
                    new_row[pos].value += 1;
                    new_row[pos].changed = true;
                    self.score += u32::pow(2, new_row[pos].value);
                    pos += 1;
                } else if new_row[pos].value == 0 {
                    new_row[pos] = tile;
                    new_row[pos].changed = false;
                } else {
                    pos += 1;
                    new_row[pos] = tile;
                    new_row[pos].changed = false;
                }
                new_row[pos].new = false;
            }
            if new_row.iter().map(|t| t.value).collect::<Vec<u32>>()
                != *row.iter().map(|t| t.value).collect::<Vec<u32>>()
            {
                changed = true
            }
            *row = new_row;
        }
        changed
    }

    pub fn move_right(&mut self) -> bool {
        for row in &mut self.board {
            row.reverse();
        }
        let changed = self.move_left();
        for row in &mut self.board {
            row.reverse();
        }
        changed
    }

    pub fn move_up(&mut self) -> bool {
        self.transpose();
        let changed = self.move_left();
        self.transpose();
        changed
    }

    pub fn move_down(&mut self) -> bool {
        self.transpose();
        let changed = self.move_right();
        self.transpose();
        changed
    }

    fn transpose(&mut self) {
        let new_board: [[Tile; SIZE]; SIZE] = (0..SIZE)
            .map(|i| {
                (0..SIZE)
                    .map(|j| self.board[j][i])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap() // Convert Vec to array
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(); // Convert Vec to array
        self.board = new_board;
    }

    pub fn is_game_over(&self) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j].value == 0 {
                    return false;
                }
                if i < SIZE - 1 && self.board[i][j].value == self.board[i + 1][j].value {
                    return false;
                }
                if j < SIZE - 1 && self.board[i][j].value == self.board[i][j + 1].value {
                    return false;
                }
            }
        }
        true
    }

    pub fn flatten(&self) -> Vec<f64> {
        let res: Vec<f64> = self
            .board
            .into_iter()
            .flat_map(|row| row.into_iter().map(|tile| tile.value as f64))
            .collect();
        res
    }

    pub fn one_hot_encode_board(&self) -> Vec<f64> {
        let mut encoded = vec![0.0; SIZE * SIZE * MAX_EXPONENT];
        for (i, &tile) in self.board.iter().flatten().enumerate() {
            let tile = tile.value as usize;
            if tile != 0 {
                encoded[i * MAX_EXPONENT + tile] = 1.0;
            }
        }
        encoded
    }

    pub fn reset(&mut self) -> Vec<f64> {
        self.board = [[Tile::new(0, 0); SIZE]; SIZE];
        self.score = 0;
        self.moves = 0;
        self.tiles = 0;
        self.add_tile();
        self.add_tile();
        self.flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{MAX_EXPONENT, SIZE};

    use super::Game;
    use super::Tile;

    fn test_game() -> Game {
        let mut game = Game::new();
        game.board = [
            [
                Tile::new(0, 0),
                Tile::new(1, 1),
                Tile::new(2, 2),
                Tile::new(3, 3),
            ],
            [
                Tile::new(4, 4),
                Tile::new(5, 5),
                Tile::new(6, 6),
                Tile::new(7, 7),
            ],
            [
                Tile::new(8, 8),
                Tile::new(9, 9),
                Tile::new(10, 10),
                Tile::new(11, 11),
            ],
            [
                Tile::new(12, 12),
                Tile::new(13, 13),
                Tile::new(14, 14),
                Tile::new(15, 15),
            ],
        ];
        game
    }

    #[test]
    fn highest_tile() {
        let game = test_game();
        assert_eq!(game.highest_tile(), Some(32768 as u32));
    }

    #[test]
    fn game_over() {
        let mut game = test_game();
        game.board[0][0].value = 16;
        assert_eq!(game.is_game_over(), true);
    }

    #[test]
    fn invalid_move() {
        let mut game = test_game();
        assert_eq!(game.move_down(), false);
        assert_eq!(game.move_right(), false);
    }

    #[test]
    fn merge_cells() {
        let mut game = test_game();

        game.board[3][2].value = 15;
        assert_eq!(game.move_right(), true);
        assert_eq!(game.highest_tile(), Some(65536 as u32));
    }

    #[test]
    fn flatten_board() {
        let game = test_game();
        let flat = game.flatten();
        assert_eq!(flat.len(), 16);
        assert_eq!(
            flat,
            (0..16).into_iter().map(|x| x as f64).collect::<Vec<f64>>()
        )
    }

    #[test]
    fn encoding() {
        let game = test_game();
        let encoding = game.one_hot_encode_board();
        assert_eq!(encoding.len(), SIZE * SIZE * MAX_EXPONENT);
        assert_eq!(encoding[0], 0.0);
        assert_eq!(encoding[MAX_EXPONENT + 1], 1.0);
        assert_eq!(encoding[MAX_EXPONENT * 4 + 4], 1.0);
        assert_eq!(encoding[MAX_EXPONENT * 15 + 15], 1.0);
    }
}
