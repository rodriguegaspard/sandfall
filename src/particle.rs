use wasm_bindgen::prelude::*;
use crate::element::ElementTable;


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Particle{
    _id: usize,
}

impl Particle{
    pub fn new(_id: usize) -> Particle {
        Particle { _id }
    }

    pub fn element(&self, elements: &ElementTable) -> String {
        elements.name(self._id).to_string()
    }
}
