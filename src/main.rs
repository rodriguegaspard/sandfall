pub mod world;
pub mod particle;
pub mod element;

use crate::world::*;
use crate::particle::Particle;
use crate::element::ElementTable;

fn main() {
    let mut w = ParticleWorld::new(Square::new(0.0, 0.0, 800.0));
    let e = ElementTable::default(); 
    let p1 = Particle::new(4);
    let p2 = Particle::new(1);
    let p3 = Particle::new(2);
    let p4 = Particle::new(3);
    w.insert(p1, 0.0, 0.0);
    w.insert(p2, 0.0, 0.0);
    w.insert(p3, 1.0, 0.0);
    w.insert(p4, 0.0, 1.0);
}
