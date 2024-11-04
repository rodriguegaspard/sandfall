use wasm_bindgen::prelude::*;

pub mod world;
pub mod particle;
pub mod element;

use crate::world::ParticleWorld;
use crate::particle::Particle;
use crate::element::ElementTable;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
fn print_test() {
    let p1 = Particle::new(0);
    let w = ParticleWorld::new(Some(p1),(0, 0, 800, 800));
    alert(&w.print_particle());
}
