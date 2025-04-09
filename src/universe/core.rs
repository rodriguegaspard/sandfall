use std::time::Instant;

use wasm_bindgen::prelude::*;

pub trait World {
    fn simulate(&mut self, delta_time: f32);
}

#[wasm_bindgen()]
pub struct Universe{
    _last_tick: Instant,
    _worlds: Vec<Box<dyn World>>
}

impl Universe{
    fn new(&self) -> Self {
        Universe {
        _last_tick: Instant::now(),
        _worlds: vec![],
        }
    }

    fn tick(&mut self){
        let delta_time = Instant::now().duration_since(self._last_tick).as_secs_f32();
        for world in &mut self._worlds{
            world.simulate(delta_time);
        }
    }
}
