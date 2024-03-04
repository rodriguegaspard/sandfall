pub mod world;
pub mod particle;

use crate::world::ParticleWorld;
use crate::particle::Particle;

fn main() {
    let p1 = Particle::new("sand".to_string());
    let w = ParticleWorld::new(Some(p1),(0, 0, 800, 800),false);
    w.print_particle();
}
