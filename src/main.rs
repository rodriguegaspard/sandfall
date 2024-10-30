pub mod world;
pub mod particle;

use crate::world::ParticleWorld;
use crate::particle::Particle;

fn main() {
    let mut w = ParticleWorld::new(None, (0, 0, 1200, 800));
    let p1 = Particle::new("water".to_string());
    let p2 = Particle::new("oil".to_string());
    let p3 = Particle::new("lava".to_string());
    let p4 = Particle::new("sand".to_string());
    w.insert(p1, 1, 1);
    w.insert(p2, 0, 0);
    w.insert(p3, 0, 1);
    w.insert(p4, 1, 0);
    println!("The element is {}", w.search(1,1).expect("Whoopsie").element());
    println!("The element is {}", w.search(0,1).expect("Whoopsie").element());
    println!("The element is {}", w.search(1,0).expect("Whoopsie").element());
    println!("The element is {}", w.search(1,1).expect("Whoopsie").element());
}
