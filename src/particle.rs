use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Particle{
    _element: String,
}

impl Particle{
    pub fn new(_element: String) -> Particle {
        Particle { _element }
    }
    pub fn element(&self) -> &String {
        &self._element
    }
}
