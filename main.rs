#[derive(Debug)]
struct Particle{
    _element: String,
}

#[derive(Debug)]
struct World{
    _particle: Option<Particle>,
    _boundaries : (u32, u32, u32, u32), //x1, y1, x2, y2

}

fn main() {
    let p1 = Particle {
        _element: "sand".to_string()
    };
    let w = World {
        _particle: Some(p1),
        _boundaries: (0, 0, 800, 800)
    };
    println!("The element of p1 is : {}", w._particle.unwrap_or(Particle { _element: ("undefined".to_string()) })._element);
}
