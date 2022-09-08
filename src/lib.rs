use std::collections::HashMap;
use std::ops::Deref;

mod util;
struct Piece {
    id: char,
    team: u8,
    flags: Vec<u8>,
}

enum Pattern {
    Repeat(i32),
    Attack(i32),
    Direction(Coord), // relative coords
    Condition(MoveCondition, Coord, bool),
    Action(MoveAction, Coord),
    Branch(Vec<Pattern>),
    Inherit(char),
}

#[derive(Debug)]
struct Coord(Vec<i32>); // differentiate between absolute and relative?

impl Coord {
    fn add(&self, other: &Coord) -> Coord {
        let (v1, v2) = match self.0.len() > other.0.len() {
            true => (self, other),
            false => (other, self),
        };
        let mut result = Vec::with_capacity(v1.0.len());
        let mut i = 0;
        loop {
            if i >= v2.0.len() {
                break;
            }
            result.push(v1.0[i] + v2.0[i]);
            i = i + 1;
        }
        loop {
            if i >= v1.0.len() {
                break;
            }
            result.push(v1.0[i]);
            i = i + 1;
        }
        Coord(result)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        let (v1, v2) = match self.0.len() > other.0.len() {
            true => (self, other),
            false => (other, self),
        };
        let mut i = 0;
        loop {
            if i >= v2.0.len() {
                break;
            }
            if v1.0[i] != v2.0[i] {
                return false;
            }
            i = i + 1;
        }
        true
    }
}

impl Eq for Coord {}

impl Deref for Coord {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum MoveCondition {
    Flag(u8),
    Team(u8),
    HasMoved,
    Piece,
    Enemy,
    Safe,
    EdgeOfBoard(u32),
}

enum MoveAction {
    Teleport(Coord),
    SetFlag(u8, bool),
    Transmute(Vec<char>),
    Destroy,
}

struct PieceType {
    patterns: Vec<Pattern>
}

pub struct Game {
    board_size: Vec<i32>,
    piece_dict: HashMap<char, PieceType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die() {
        // fuck you lmaooooo
        let a = Coord(vec![1, 2, 3, 0]);
        assert_eq!(a, Coord(vec![1, 2, 3]));
    }
}
