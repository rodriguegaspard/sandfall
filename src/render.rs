use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::grid::*;
use crate::particle::Particle;

pub trait Renderer {
    // Change this into generic
    fn draw (&self, world: &Vec<Particle>, coords: &Vec<usize>, context: &CanvasRenderingContext2d);
}

#[wasm_bindgen]
pub struct ParticleRenderer;
impl ParticleRenderer{
    fn extract_coords(&self, index: &usize) -> (f64, f64){
        return ((index % WIDTH) as f64, (index / WIDTH) as f64)
    }
}
impl Renderer for ParticleRenderer{
    fn draw (&self, world: &Vec<Particle>, coords: &Vec<usize>, context: &CanvasRenderingContext2d) {
        for index in coords {
            let coord = self.extract_coords(index);
            context.set_fill_style_str("blue");
            context.fill_rect(coord.0, coord.1, 1.0, 1.0);
        }
    }
}
