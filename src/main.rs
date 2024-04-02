pub mod world;
pub mod particle;

use crate::world::ParticleWorld;
use crate::particle::Particle;

fn main() {
    let mut w = ParticleWorld::new(None, (0, 0, 800, 800));
    let p = Particle::new("water".to_string());
    w.insert(p, 100, 100);
}
