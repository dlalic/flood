pub mod fill;

use crate::fill::fill_depressions::fill_depressions;
use crate::fill::point::Point;
use num_rational::Rational64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Landscape {
    elevations: Vec<Point>,
    current_elevations: Vec<Point>,
    water_levels: Vec<Rational64>,
}

impl Default for Landscape {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Landscape {
    pub fn new() -> Landscape {
        Landscape {
            elevations: vec![],
            current_elevations: vec![],
            water_levels: vec![],
        }
    }

    pub fn set_elevations(&mut self, input: &str) {
        let elevations: Vec<Point> = input
            .split(',')
            .enumerate()
            .map(|(index, value)| Point {
                index: index as u64,
                elevation: Rational64::from_integer(value.parse().unwrap()),
            })
            .collect();
        self.elevations = elevations;
        self.current_elevations = vec![];
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
        if self.current_elevations.is_empty() {
            self.water_levels = fill_depressions(&self.elevations);
        } else {
            self.water_levels = fill_depressions(&self.current_elevations);
        }
        self.current_elevations = self
            .water_levels
            .iter()
            .enumerate()
            .map(|(index, elevation)| Point {
                index: index as u64,
                elevation: *elevation,
            })
            .collect();
    }
}
