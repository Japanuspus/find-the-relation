use std::rc::Rc;

use crate::group::{Group, Direction};
use crate::algebra::{Matrix};
use num::Complex;

pub struct Move {
    direction: Direction,
    parent: Option<Rc<Move>>,
    generation: usize,
}

pub struct LeafState {
    pub state: Vec<Matrix>,
    last_move: Rc<Move>,
}

pub struct Level {
    pub qs: Vec<Complex<f64>>,
    pub groups: Vec<Group>,
}

fn make_leaf_child(leaf: &LeafState, direction: Direction, groups: &Vec<Group>) -> LeafState {
    let state = groups.iter().zip(leaf.state.iter()).map(|(g, s)| g.apply(s, direction)).collect();
    let last_move = Rc::new(Move {
        direction,
        parent: Some(leaf.last_move.clone()),
        generation: leaf.last_move.generation + 1,
    });
    LeafState{state, last_move}                
}

impl Level {
    pub fn new(qs: Vec<Complex<f64>>) -> Level {
        let groups = qs.iter().map(Group::new).collect();
        Level {qs, groups}
    }

    pub fn og(&self) -> impl Iterator<Item=LeafState> + '_{
        Direction::iter().map(move |direction| LeafState {
            state: self.groups.iter().map(|g| g.apply(&g.identity, direction)).collect(),
            last_move: Rc::new(Move{direction, parent: None, generation: 1}),
        })
    }

    pub fn get_child(&self, leaf: &LeafState, direction: Direction) -> LeafState {
        make_leaf_child(leaf, direction, &self.groups)
    }

    pub fn get_children<'a>(&'a self, leaf: &'a LeafState) -> impl Iterator<Item=LeafState> + 'a {
        let d0 = leaf.last_move.direction;
        Direction::iter().filter_map(move |direction| {
            if direction.is_opposite(d0) {None} else {Some(self.get_child(leaf, direction))}
        })
    }
}

impl LeafState {
    pub fn recipe(&self) -> Vec<Direction> {
        let mut res = Vec::new();
        let mut next_move = &Some(self.last_move.clone());
        while let Some(m) = next_move {
            res.push(m.direction);
            next_move = &m.parent;
        }
        res.reverse();
        res
    }

    pub fn is_solution(&self) -> bool {
        self.state.iter().all(|m| m.is_identity())
    }

    pub fn length(&self) -> usize {
        self.last_move.generation
    }
}

/*

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_word_updates_as_expected() {
        let q = Complex::new(60.0, 42.0);
        let mut level = Level::new(vec![q]);

        assert_eq!(level.word, vec![]);
        assert_eq!(level.word(), "");

        level.push(Direction::North);
        assert_eq!(level.word, vec![Direction::North]);
        assert_eq!(level.word(), "N");

        level.push(Direction::East);
        assert_eq!(level.word, vec![Direction::North, Direction::East]);
        assert_eq!(level.word(), "NE");

        level.push(Direction::West);
        assert_eq!(level.word, vec![Direction::North]);
        assert_eq!(level.word(), "N");

        level.push(Direction::South);
        assert_eq!(level.word, vec![]);
        assert_eq!(level.word(), "");
    }

    #[test]
    fn level_reset_resets() {
        let q = Complex::new(60.0, 42.0);
        let mut level = Level::new(vec![q]);

        assert!(level.groups[0].current_is_identity());

        level.push(Direction::North);
        assert!(!level.word.is_empty());
        assert!(!level.groups[0].current_is_identity());

        level.reset();
        assert!(level.word.is_empty());
        assert!(level.groups[0].current_is_identity());
    }

    #[test]
    fn level_is_solved() {
        let q = Complex::new(1.0, 0.0);
        let mut level = Level::new(vec![q]);

        assert!(!level.is_solved());
        level.push(Direction::North);
        assert!(!level.is_solved());
        level.push(Direction::North);
        assert!(level.is_solved());
    }
}
*/