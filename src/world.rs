use wasm_bindgen::prelude::*;
use crate::Particle;

#[wasm_bindgen]
#[derive(Debug)]
pub struct ParticleWorld{
    _particle: Option<Particle>,
    _boundaries : (u32, u32, u32, u32), //x1, y1, x2, y2 (top-left and bottom-right corner)
    _is_leaf: bool,
}

impl ParticleWorld {
    pub fn new(_particle: Option<Particle>, _boundaries: (u32, u32, u32, u32), _is_leaf: bool) -> ParticleWorld {
        ParticleWorld { _particle, _boundaries, _is_leaf }
    }
    pub fn print_particle(&self) -> String {
        match &self._particle{
            Some(particle) => format!("{}{}", "This particle is made of ", particle.element()),
            None => "".to_string(),
        }
    }
}
