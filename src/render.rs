use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::particle::core::Particle;
use crate::universe::grid::*;

pub trait Renderer {
    // Change this into generic
    fn draw (&self, world: &Vec<Particle>, coords: &Vec<usize>, context: &CanvasRenderingContext2d);
}

#[wasm_bindgen]
pub struct ParticleRenderer;

impl Renderer for ParticleRenderer{
    fn draw (&self, world: &Vec<Particle>, coords: &Vec<usize>, context: &CanvasRenderingContext2d) {
            context.clear_rect(0.0, 0.0, 800.0, 800.0);
            context.set_fill_style_str("black");
            context.fill_rect(0.0, 0.0, 800.0, 800.0);
        for index in coords {
            context.set_fill_style_str("blue");
            context.fill_rect((index % WIDTH) as f64, (index / HEIGHT) as f64, 5.0, 5.0);
        }
    }
}
