pub mod world;
pub mod particle;

use crate::world::ParticleWorld;
use crate::particle::Particle;

fn main() {
    let mut w = ParticleWorld::new(None, (0, 0, 800, 800));
    let p1 = Particle::new("water".to_string());
    let p2 = Particle::new("oil".to_string());
    let p3 = Particle::new("lava".to_string());
    let p4 = Particle::new("sand".to_string());
    w.insert(p1, 0, 0);
    w.insert(p2, 0, 799);
    w.insert(p3, 799, 0);
    w.insert(p4, 799, 799);
}
