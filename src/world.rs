use wasm_bindgen::prelude::*;
use crate::Particle;

#[wasm_bindgen]
#[derive(Debug)]
pub struct ParticleWorld{
    _particle: Option<Particle>,
    _boundaries : (u32, u32, u32, u32), //x1, y1, x2, y2 (top-left and bottom-right corner)
    _quadrants: [Option<Box<ParticleWorld>>; 4]
}

impl ParticleWorld {
    pub fn new(_particle: Option<Particle>, _boundaries: (u32, u32, u32, u32)) -> Self {
        ParticleWorld {
            _particle,
            _boundaries,
            _quadrants: [None, None, None, None]
        }
    }

    pub fn print_particle(&self) -> String {
        match &self._particle{
            Some(particle) => format!("{}{}", "This particle is made of ", particle.element()),
            None => "".to_string(),
        }
    }

    pub fn print_bounds(&self) {
        println!("The boundaries of this quadrant are : ({};{}) and ({};{})", &self._boundaries.0, &self._boundaries.1, &self._boundaries.2, &self._boundaries.3);
        for q in &self._quadrants {
            if let Some(child) = q {
                child.print_bounds();
            }
        }
    }

    pub fn split_tree(&mut self) {
            self._quadrants[0] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0, self._boundaries.1, self._boundaries.2/2, self._boundaries.3/2))));
            self._quadrants[1] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0+self._boundaries.2/2, self._boundaries.1, self._boundaries.2, self._boundaries.3/2))));
            self._quadrants[2] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0, self._boundaries.1+self._boundaries.3/2, self._boundaries.2/2, self._boundaries.3))));
            self._quadrants[3] = Some(Box::new(ParticleWorld::new(None, (self._boundaries.0+self._boundaries.2/2, self._boundaries.1+self._boundaries.3/2, self._boundaries.2, self._boundaries.3))));
    }

    pub fn contains_coords(&self, x: u32, y: u32) -> bool {
        (self._boundaries.0 <= x) && (self._boundaries.1 <= y) && (self._boundaries.2 >= x) && (self._boundaries.3 >= y)
    } 
}

