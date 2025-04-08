use wasm_bindgen::prelude::*;

pub trait World {
    fn simulate(&mut self);
}

#[wasm_bindgen()]
pub struct Universe{
    _worlds: Vec<Box<dyn World>>
}

impl Universe{
    fn tick(&mut self){
        for world in &mut self._worlds{
            world.simulate();
        }
    }
}
