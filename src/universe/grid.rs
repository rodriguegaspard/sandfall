use wasm_bindgen::prelude::*;
use crate::particle::velocity::Gravity;
use crate::particle::Particle;
use crate::universe::core::World;

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
        &self._current
    }

    pub fn active_particles(&self) -> &Vec<usize> {
        &self._current_active_particles
    }

    fn is_within_bounds(&self, x: usize, y: usize) -> bool{
        x <= HEIGHT && y <= WIDTH 
    }

    fn is_empty(&self, x: usize, y: usize) -> bool{
        self._current[y * WIDTH + x].id() == usize::MAX
    }

    fn is_valid(&self, x: usize, y: usize) -> bool{
        self.is_within_bounds(x, y) && self.is_empty(x, y)
    }

    fn add_active_particle(&mut self, index: usize) {
        self._current_active_particles.push(index);
    }

    pub fn insert(&mut self, x: usize, y: usize, particle: Particle) -> bool { 
        if self.is_valid(x, y){
            self._current[y * WIDTH + x] = particle;
            self.add_active_particle(y * WIDTH + x);
            return true;
        }
        false
    }

    pub fn delete(&mut self, x: usize, y: usize) -> bool {
        if self.is_valid(x, y){
            self._current[y * WIDTH + x].delete(true);
            self._current_active_particles.remove(self._current_active_particles.iter().position(|i| *i == y * WIDTH + x).expect("index not found"));
            return true;
        }
        false
    }

    pub fn shift(&mut self, x: usize, y: usize, delta_time: f32){
        let tmp = self._current[y * WIDTH + x];
        self.delete(x, y);
        let new_x = (x as f32 + tmp.velocity().x() * delta_time) as usize;
        let new_y = (y as f32 + tmp.velocity().y() * delta_time) as usize;
        if self.is_within_bounds(new_x, new_y){
            self._next[new_y * WIDTH + x] = tmp;
            self._next_active_particles.push(new_y * WIDTH + x);
        }
    }

    pub fn replace(&mut self, x: usize, y: usize, particle: Particle){
        if self.is_within_bounds(x, y){
            self._current[y * WIDTH + x] = particle;
        }
    }
}

impl World for ParticleWorld {
    fn simulate(&mut self, delta_time: f32) {
        self._next.fill(Particle::empty());
        self._next_active_particles.clear();
        for index in self._current_active_particles.clone() {
            if self._current[index].is_deleted(){
                // do nothing
            }
            else {
                self._current[index].apply_gravity();
                self.shift(index % WIDTH, index / WIDTH, delta_time);
            }
        }
        std::mem::swap(&mut self._current, &mut self._next);
        std::mem::swap(&mut self._current_active_particles, &mut self._next_active_particles);
            }
}

#[cfg(test)]
mod tests{
    use super::*;

#[test]
    fn insert_active_particle() {
        let mut pw = ParticleWorld::new();
        let p1 = Particle::new(100);
        pw.insert(100, 100, p1);
        assert_eq!(pw._current_active_particles.len(), 1);
    }

#[test]
    fn correct_particle_placement() {
        let mut pw = ParticleWorld::new();
        let p1 = Particle::new(100);
        pw.insert(100, 100, p1);
        assert_eq!(pw._current_active_particles[0], 80100);
    }

#[test]
    fn update_particle_velocity() {
        let mut pw = ParticleWorld::new();
        let p1 = Particle::new(100);
        pw.insert(100, 100, p1);
        pw.simulate(10.0);
        assert_eq!(pw._current_active_particles[0],(100.0 + 0.0098 * 10.0) as usize * 800 + (100.0 + 0.0098 * 10.0) as usize);
    }
}
