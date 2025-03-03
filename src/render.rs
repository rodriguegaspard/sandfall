use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::world::Square;

pub trait Renderer {
    fn draw (&self, data: &Vec<Square>, context: &CanvasRenderingContext2d);
}

#[wasm_bindgen]
pub struct ParticleRenderer;
impl Renderer for ParticleRenderer{
    fn draw (&self, data: &Vec<Square>, context: &CanvasRenderingContext2d) {
        for point in data {
            context.fill_rect(point.x(), point.y(), point.unit(), point.unit());
        }
    }
}

#[wasm_bindgen]
pub struct QuadrantRenderer;
impl Renderer for QuadrantRenderer {
    fn draw (&self, data: &Vec<Square>, context: &CanvasRenderingContext2d) {
        for quadrant in data {
            context.stroke_rect(quadrant.x(), quadrant.y(), quadrant.unit(), quadrant.unit());
        }
    }
}
