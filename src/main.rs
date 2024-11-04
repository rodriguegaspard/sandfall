pub mod world;
pub mod particle;
pub mod element;

use crate::world::ParticleWorld;
use crate::particle::Particle;

fn main() {
    let mut w = ParticleWorld::new(None, (0, 0, 1200, 800));
    let p1 = Particle::new(0);
    let p2 = Particle::new(1);
    let p3 = Particle::new(2);
    let p4 = Particle::new(3);
    w.insert(p1, 1, 1);
    w.insert(p2, 0, 0);
    w.insert(p3, 0, 1);
    w.insert(p4, 1, 0);
    w.search(1,1).expect("Whoopsie");
    w.search(0,1).expect("Whoopsie");
    w.search(1,0).expect("Whoopsie");
    w.search(1,1).expect("Whoopsie");
}
