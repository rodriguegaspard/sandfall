use wasm_bindgen::prelude::*;
use crate::particle::Particle;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

#[wasm_bindgen]
pub struct ParticleWorld{
    _current: Vec<Particle>,
    _next : Vec<Particle>,
    _current_active_particles : Vec<usize>,
    _next_active_particles: Vec<usize>
}

impl ParticleWorld {
    pub fn new() -> Self {
        Self {
            _current: vec![Particle::empty(); WIDTH * HEIGHT],
            _next: vec![Particle::empty(); WIDTH * HEIGHT],
            _current_active_particles: vec![],
            _next_active_particles: vec![],
        }
    }

    pub fn grid(&self) -> &Vec<Particle> {
        &self._next
    }

    pub fn active_particles(&self) -> &Vec<usize> {
        &self._next_active_particles
    }

    fn is_within_bounds(&self, x: usize, y: usize) -> bool{
        x >= 0 && x <= HEIGHT && y >=0 && y <= WIDTH 
    }

    fn is_empty(&self, x: usize, y: usize) -> bool{
        self._current[y * WIDTH + x].id() == usize::MAX
    }

    fn is_valid(&self, x: usize, y: usize) -> bool{
        self.is_within_bounds(x, y) && self.is_empty(x, y)
    }

    fn add_active_particle(&mut self, index: usize) {
        self._next_active_particles.push(index);
    }

    pub fn insert(&mut self, x: usize, y: usize, particle: Particle) -> bool { 
        if self.is_valid(x, y){
            self._next[y * WIDTH + x] = particle;
            self.add_active_particle(y * WIDTH + x);
            return true;
        }
        false
    }

    pub fn delete(&mut self, x: usize, y: usize) -> bool {
        if self.is_valid(x, y){
            self._current[y * WIDTH + x].delete();
            return true;
        }
        false
    }

    pub fn shift(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        if self.is_valid(x1, y1){
            let temp = self._current[y1 * WIDTH + x1];
            return self.insert(x2, y2, temp);
        }
        false
    }

    pub fn replace(&mut self, x: usize, y: usize, particle: Particle){
        if self.is_within_bounds(x, y){
            self._next[y * WIDTH + x] = particle;
        }
    }
}
