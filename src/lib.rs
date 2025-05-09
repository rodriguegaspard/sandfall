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
use web_sys::window;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent};

use std::cell::RefCell;
use std::rc::Rc;

struct AppState {
    mouse_x: f64,
    mouse_y: f64,
    is_drawing: bool,
}

fn start_animation_loop(
    context: CanvasRenderingContext2d,
    state: Rc<RefCell<AppState>>,
    world : Rc<RefCell<ParticleWorld>>,
    renderer: ParticleRenderer,
    document: web_sys::Document,
) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut last_time = window().unwrap().performance().unwrap().now();
    let mut accumulator = 0.0;
    const TICK_RATE: f64 = 1000.0 / 60.0; // ~16.6667ms per frame

    let closure = Closure::wrap(Box::new(move || {
        let state = state.borrow();

        if state.is_drawing {
            world.borrow_mut().insert(state.mouse_x as usize, state.mouse_y as usize, Particle::new(100));
        }

        let now = window().unwrap().performance().unwrap().now();
        let delta = now - last_time;
        last_time = now;

        accumulator += delta;

        // Add timers for simulation and rendering, might be interesting

        while accumulator >= TICK_RATE {
            world.borrow_mut().simulate(delta as f32);
            accumulator -= TICK_RATE;
            let fps = format!("{:05.2} fps", (1.0 / delta) * 1000.0);
            document.get_element_by_id("fps_counter").unwrap().set_inner_html(&fps);
        }
        renderer.draw(world.borrow().grid(), world.borrow().active_particles(), &context);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>);

    *g.borrow_mut() = Some(closure);
    request_animation_frame(g.borrow().as_ref().unwrap());
}

// Utility function to wrap requestAnimationFrame
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let w = Rc::new(RefCell::new(ParticleWorld::new()));
    let r = ParticleRenderer;
    
    let state = Rc::new(RefCell::new(AppState {
        mouse_x: 0.0,
        mouse_y: 0.0,
        is_drawing: false,
    }));

    let canvas_clone = canvas.clone();
    let state_clone = state.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        let rect = canvas_clone
            .get_bounding_client_rect();

        let mut state = state_clone.borrow_mut();
        state.mouse_x = event.client_x() as f64 - rect.left();
        state.mouse_y = event.client_y() as f64 - rect.top();
    });
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Same idea for mousedown:
    let state_clone = state.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        state_clone.borrow_mut().is_drawing = true;
    });
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // And mouseup:
    let state_clone = state.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        state_clone.borrow_mut().is_drawing = false;
    });
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();

    start_animation_loop(context.clone(), state.clone(), w, r, document);
    
    Ok(())
}
