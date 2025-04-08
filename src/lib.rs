pub mod particle;
pub mod universe;
pub mod render;

use particle::Particle;
use render::ParticleRenderer;
use render::Renderer;
use universe::core::World;
use universe::ParticleWorld;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};

use std::cell::RefCell;
use std::rc::Rc;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let body = document.body().expect("document should have a body");
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut w = ParticleWorld::new();
    let r = ParticleRenderer;
    let p1 = Particle::new(1);
    let p2 = Particle::new(2);
    let p3 = Particle::new(3);
    let p4 = Particle::new(4);
    w.insert(100, 200, p1);
    w.insert(300, 500, p2);
    w.insert(700, 100, p3);
    w.insert(400, 700, p4);
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let ctx = Rc::new(context);

    let mut i = 0.0;
    *g.borrow_mut() = Some(Closure::new(move || {
        if i > 200.0 {
            body.set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        i += 1.0;
        w.simulate(i);
        r.draw(w.grid(), w.active_particles(), &ctx);
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
