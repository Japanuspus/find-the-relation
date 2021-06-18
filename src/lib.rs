mod algebra;
mod group;

use std::f64::consts::PI;

use js_sys::{Array};
use num::{Complex};
use num::integer::{div_mod_floor};
use wasm_bindgen::prelude::*;
use crate::group::{Direction, Group};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    groups: [Group; 3],
    evaluated: [[Complex<f64>; 9]; 3]
}

#[wasm_bindgen]
impl Game {
    pub fn easy() -> Game {
        let groups = [
            Group::new(&Complex::new(1.0, 0.0)),
            Group::new(&Complex::new(0.5, 0.0)),
            Group::new(&Complex::from_polar(1.0, PI/5.0))
            ];
        let evaluated = [groups[0].flatten(), groups[1].flatten(), groups[2].flatten()];
        Game {
            groups,
            evaluated
        }
    }

    pub fn push(&mut self, direction: Direction) {
        for i in 0..3 {
            self.groups[i].push(&direction);
            self.evaluated[i] = self.groups[i].flatten();
        }
    }

    pub fn evaluated(&self) -> Array {
        let arr = Array::new_with_length(27);
        for i in 0..27 {
            let (j, k) = div_mod_floor(i, 9);
            let s = JsValue::from_str(&format!("{}", self.evaluated[j][k]));
            arr.set(i as u32, s);
        }
        arr
    }

    pub fn evaluated_polar(&self) -> Array {
        let arr = Array::new_with_length(27);
        for i in 0..27 {
            let (j, k) = div_mod_floor(i, 9);
            let (r, theta) =  self.evaluated[j][k].to_polar();
            let s = JsValue::from_str(&format!("({}, {})", r, theta));
            arr.set(i as u32, s);
        }
        arr
    }

    pub fn evaluation_is_trivial(&self) -> Array {
        let arr = Array::new_with_length(3);
        for i in 0..3 {
            arr.set(i as u32, JsValue::from_bool(self.groups[i].current_is_identity()));
        }
        arr
    }
}