pub mod fill;

use crate::fill::fill_depressions::fill_depressions;
use crate::fill::point::Point;
use num_rational::Rational64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Landscape {
    elevations: Vec<Point>,
    water_levels: Vec<Rational64>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Landscape {
    pub fn new(input: &str) -> Landscape {
        let elevations: Vec<Point> = input
            .split(',')
            .enumerate()
            .map(|(index, value)| Point {
                index: index as u64,
                elevation: Rational64::from_integer(value.parse().unwrap()),
            })
            .collect();
        Self {
            elevations,
            water_levels: vec![],
        }
    }

    pub fn elevations_count(&self) -> usize {
        self.elevations.len()
    }

    pub fn elevation(&self, index: usize) -> f64 {
        let point = &self.elevations[index];
        *point.elevation.numer() as f64 / *point.elevation.denom() as f64
    }

    pub fn water_level(&self, index: usize) -> f64 {
        let point = &self.elevations[index];
        let result = self.water_levels[index] - point.elevation;
        *result.numer() as f64 / *result.denom() as f64
    }

    pub fn update(&mut self) {
        self.water_levels = fill_depressions(&self.elevations);
    }
}
