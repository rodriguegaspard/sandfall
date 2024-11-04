use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Particle{
    _id: u8,
}

impl Particle{
    pub fn new(_id: u8) -> Particle {
        Particle { _id }
    }

    pub fn element(&self) -> String {
        String::from("Placeholder")
    }
}
