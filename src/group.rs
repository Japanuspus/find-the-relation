use std::fmt::Write;

use crate::algebra::{evaluate_polynomial, Matrix};
use num::complex::Complex;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match (self, other) {
            (Direction::North, Direction::South) => true,
            (Direction::South, Direction::North) => true,
            (Direction::East, Direction::West) => true,
            (Direction::West, Direction::East) => true,
            _ => false,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::South, Direction::East, Direction::West].iter().copied()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        })
    }
}

pub struct Group {
    north_matrix: Matrix,
    south_matrix: Matrix,
    east_matrix: Matrix,
    west_matrix: Matrix,
    pub identity: Matrix,
}

impl Group {
    pub fn new(q: &Complex<f64>) -> Self {
        // See https://arxiv.org/abs/1904.11730v3
        let mut north_matrix = Matrix::zero();
        north_matrix.d[0][2] = evaluate_polynomial(&[(-1, -1)], &q);
        north_matrix.d[1][1] = evaluate_polynomial(&[(1, -1)], &q);
        north_matrix.d[1][2] = evaluate_polynomial(&[(-1, -1), (1, 1)], &q);
        north_matrix.d[2][0] = evaluate_polynomial(&[(0, -1)], &q);
        north_matrix.d[2][2] = evaluate_polynomial(&[(-1, -1), (0, 1)], &q);

        let mut south_matrix = Matrix::zero();
        south_matrix.d[0][0] = evaluate_polynomial(&[(0, 1), (1, -1)], &q);
        south_matrix.d[0][2] = evaluate_polynomial(&[(0, -1)], &q);
        south_matrix.d[1][0] = evaluate_polynomial(&[(-1, 1), (1, -1)], &q);
        south_matrix.d[1][1] = evaluate_polynomial(&[(-1, -1)], &q);
        south_matrix.d[2][0] = evaluate_polynomial(&[(1, -1)], &q);

        let mut east_matrix = Matrix::zero();
        east_matrix.d[0][0] = evaluate_polynomial(&[(-1, -1)], &q);
        east_matrix.d[0][1] = evaluate_polynomial(&[(0, 1)], &q);
        east_matrix.d[1][1] = evaluate_polynomial(&[(0, 1)], &q);
        east_matrix.d[2][1] = evaluate_polynomial(&[(0, 1)], &q);
        east_matrix.d[2][2] = evaluate_polynomial(&[(1, -1)], &q);

        let mut west_matrix = Matrix::zero();
        west_matrix.d[0][0] = evaluate_polynomial(&[(1, -1)], &q);
        west_matrix.d[0][1] = evaluate_polynomial(&[(1, 1)], &q);
        west_matrix.d[1][1] = evaluate_polynomial(&[(0, 1)], &q);
        west_matrix.d[2][1] = evaluate_polynomial(&[(-1, 1)], &q);
        west_matrix.d[2][2] = evaluate_polynomial(&[(-1, -1)], &q);

        Self { north_matrix, south_matrix, east_matrix, west_matrix, identity: Matrix::identity()}
    }

    pub fn get(&self, direction: Direction) -> &Matrix {
        match direction {
            Direction::North => &self.north_matrix,
            Direction::South => &self.south_matrix,
            Direction::East => &self.east_matrix,
            Direction::West => &self.west_matrix
        }
    }

    pub fn apply(&self, state: &Matrix, direction: Direction) -> Matrix {
        state * self.get(direction)
    }

    pub fn is_identity(&self, state: &Matrix) -> bool {
        state == &self.identity
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_starts_at_identity() {
        // let q = Complex::new(60.0, 42.0);
        // let group = Group::new(&q);
        let state = Matrix::identity();
        assert!(state.is_identity())
    }

    #[test]
    fn group_moves_to_non_identity() {
        let q = Complex::new(60.0, 42.0);
        let group = Group::new(&q);
        let mut state = Matrix::identity();
        state = group.apply(&state, Direction::North);
        assert!(!state.is_identity());
    }

    /*
    #[test]
    fn pushing_north_moves_north() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, group.north_matrix);
    }

    #[test]
    fn going_north_and_south_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::North);
        group.push(&Direction::South);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_south_and_north_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::South);
        group.push(&Direction::North);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_east_and_west_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::East);
        group.push(&Direction::West);
        assert_eq!(group.current_matrix, Matrix::identity());
    }

    #[test]
    fn going_west_and_east_does_nothing() {
        let q = Complex::new(60.0, 42.0);
        let mut group = Group::new(&q);
        group.push(&Direction::West);
        group.push(&Direction::East);
        assert_eq!(group.current_matrix, Matrix::identity());
    }
    */
}