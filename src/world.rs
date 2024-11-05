// Definition of the ParticleWorld class, which is a quadtree that holds Particle objects. The methods
// are basic quadtree operations, such as insert, range search, delete and rebalancing.

use wasm_bindgen::prelude::*;
use crate::{element::ElementTable, Particle};

#[wasm_bindgen]
pub struct ParticleWorld{
    _particle: Option<Particle>,
    _boundaries : (u32, u32, u32, u32), //x1, y1, x2, y2 (top-left and bottom-right corner)
    _quadrants: [Option<Box<ParticleWorld>>; 4],
}

impl ParticleWorld {
    pub fn new(_particle: Option<Particle>, _boundaries: (u32, u32, u32, u32)) -> Self {
        ParticleWorld {
            _particle,
            _boundaries,
            _quadrants: [None, None, None, None],
        }
    }

    pub fn print_particle(&self, elements: &ElementTable) -> String {
        match &self._particle{
            Some(particle) => format!("This particle is made of {} and is at ({};{})({};{})" , particle.element(elements), &self._boundaries.0, &self._boundaries.1, &self._boundaries.2, &self._boundaries.3),
            None => "Nothing!".to_string(),
        }
    }

    pub fn print_bounds(&self) -> String {
        format!("The boundaries of this quadrant are : ({};{}) and ({};{})", &self._boundaries.0, &self._boundaries.1, &self._boundaries.2, &self._boundaries.3)
    }

    pub fn split_tree(&mut self) {
        if self.is_leaf(){
            let ax = (self._boundaries.0 + self._boundaries.2) / 2;
            let ay = (self._boundaries.1 + self._boundaries.3) / 2;
            self._quadrants[0] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0, self._boundaries.1, ax, ay))));
            self._quadrants[1] = Some(Box::new(ParticleWorld::new(None, (ax, self._boundaries.1, self._boundaries.2, ay))));
            self._quadrants[2] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0, ay, ax, self._boundaries.3))));
            self._quadrants[3] = Some(Box::new(ParticleWorld::new(None, (ax, ay, self._boundaries.2, self._boundaries.3))));
        }
    }

    pub fn is_leaf(&self) -> bool {
        self._quadrants[0].is_none()
    }

    pub fn contains_coords(&self, x: u32, y: u32) -> bool {
        (self._boundaries.0 <= x) && (self._boundaries.1 <= y) && (self._boundaries.2 > x) && (self._boundaries.3 > y)
    }

    pub fn is_at_max_depth(&self) -> bool {
        (self._boundaries.0 + 1 == self._boundaries.2) && (self._boundaries.1 + 1 == self._boundaries.3)
    }

    pub fn insert(&mut self, particle: Particle, x: u32, y: u32) -> bool {
        if !self.contains_coords(x, y){
            self.print_bounds();
            false
        }
        else if self.is_at_max_depth() && self._particle.is_none(){
            self._particle = Some(particle);
            true
        }
        else{
            if self.is_leaf(){
                self.split_tree();
            }
            for child in &mut self._quadrants {
                if child.as_mut().unwrap().contains_coords(x, y){
                    return child.as_mut().unwrap().insert(particle, x, y);
                }
            }
            false
        }

    }

    pub fn search(&self, x: u32, y: u32) -> Option<Particle> {
        if self.is_at_max_depth(){
            self._particle.clone()
        }
        else{
            for child in &self._quadrants {
                if child.as_ref().unwrap().contains_coords(x, y){
                    return child.as_ref().unwrap().search(x, y);
                }
            }
            None
        }
    }
}

