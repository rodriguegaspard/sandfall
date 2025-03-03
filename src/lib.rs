pub mod world;
pub mod particle;
pub mod element;
pub mod render;

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use crate::world::{ParticleWorld, Square};
use crate::particle::Particle;
use crate::render::Renderer;
use render::QuadrantRenderer;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // HTML <canvas> initialization
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // World initialization
    let mut w = ParticleWorld::new(Square::new(0.0, 0.0, 800.0));
    let p = Particle::new(1);
    w.insert(p, 250.0, 350.0);
    let mut quadrants = Vec::new(); 
    w.get_quadrants(&mut quadrants);
    let qr = QuadrantRenderer;
    qr.draw(&quadrants, &context);
    Ok(())
}
