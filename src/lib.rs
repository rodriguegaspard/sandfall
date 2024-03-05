use wasm_bindgen::prelude::*;

pub mod world;
pub mod particle;

use crate::world::ParticleWorld;
use crate::particle::Particle;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
fn print_test() {
    let p1 = Particle::new("sand".to_string());
    let w = ParticleWorld::new(Some(p1),(0, 0, 800, 800),false);
    alert(&w.print_particle());
}
