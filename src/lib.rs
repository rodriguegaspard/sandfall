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
    let mut w = ParticleWorld::new(Square::new(0.0, 0.0, 800.0));
    let p1 = Particle::new(1);
    let p2 = Particle::new(1);
    let p3 = Particle::new(1);
    let p4 = Particle::new(1);
    let p5 = Particle::new(1);
    let p6 = Particle::new(1);
    let p7 = Particle::new(1);
    w.insert(p1, 250.0, 350.0);
    w.insert(p2, 150.0, 350.0);
    w.insert(p3, 750.0, 450.0);
    w.insert(p4, 0.0,    50.0);
    w.insert(p5, 549.0, 40.0);
    w.insert(p6, 21.0, 12.0);
    w.insert(p7, 15.0, 700.0);
    let mut quadrants = Vec::new(); 
    w.get_quadrants(&mut quadrants);
    let qr = QuadrantRenderer;
    qr.draw(&quadrants, &context);
    let mut points = Vec::new(); 
    w.get_particles(&mut points);
    let pr = ParticleRenderer;
    pr.draw(&points, &context);
    Ok(())
}
