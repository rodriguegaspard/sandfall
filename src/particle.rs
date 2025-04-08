use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Particle{
    _id: usize,
    _updated: bool,
    _deleted: bool
}

impl Particle{
    pub fn new(_id: usize) -> Particle {
        Particle { 
            _id, 
            _updated: false, 
            _deleted: false
        }
    }

    pub fn empty() -> Particle {
        Particle {
        _id: usize::MAX,
        _updated: false,
        _deleted: false
        }
    }

    pub fn id(&self) -> usize {
        self._id
    }

    pub fn is_updated(&self) -> bool {
        self._updated
    }

    pub fn is_deleted(&self) -> bool {
        self._deleted
    }

    pub fn update(&mut self){
        self._updated = !self._updated;
    }

    pub fn delete(&mut self){
        self._deleted = true
    }
}
