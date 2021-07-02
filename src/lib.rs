mod algebra;
mod group;
mod level;

use std::f64::consts::PI;
use num::Complex;

pub use crate::algebra::Matrix;
pub use crate::level::{Level, LeafState};
pub use crate::group::{Direction, Group};

pub struct Game {
    pub levels: Vec<Level>,
}

impl Game {
    pub fn new() -> Game {
        let mut levels = vec![];
        levels.push(Level::new(vec![Complex::new(1.0, 0.0)]));
        levels.push(Level::new(vec![Complex::new(-1.0, 0.0)]));
        levels.push(Level::new(vec![Complex::new(0.0, 1.0)]));
        levels.push(Level::new(vec![Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(0.0, 1.0)]));
        levels.push(Level::new(vec![Complex::from_polar(1.0, 2.0 * PI / 3.0)]));
        levels.push(Level::new(vec![Complex::from_polar(1.0, 2.0 * PI / 5.0)]));
        levels.push(Level::new(vec![Complex::from_polar(1.0, 2.0 * PI / 3.0), Complex::from_polar(1.0, 2.0 * PI / 5.0)]));
        levels.push(Level::new(vec![Complex::new(2.0, 0.0)]));
        levels.push(Level::new(vec![Complex::new(3.0, 0.0)]));
        Game { levels }
    }
}
