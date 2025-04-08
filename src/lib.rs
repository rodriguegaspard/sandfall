pub mod render;
pub mod universe;
pub mod events;
pub mod particle;

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use crate::particle::core::Particle;
use crate::universe::grid::ParticleWorld;
use crate::render::Renderer;
use render::ParticleRenderer;

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
    let mut pw = ParticleWorld::new();
    let pr = ParticleRenderer;
    let p1 = Particle::new(0);
    let p2 = Particle::new(1);
    let p3 = Particle::new(2);
    let p4 = Particle::new(3);
    pw.insert(100, 100, p1);
    pw.insert(200, 0, p2);
    pw.insert(300, 400, p3);
    pw.insert(1000, 100, p4);
    pr.draw(pw.grid(), pw.active_particles(), &context);
    Ok(())
}
