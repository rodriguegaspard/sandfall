use wasm_bindgen::prelude::*;
use super::velocity::Force;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Particle{
    _id: usize,
    _updated: bool,
    _deleted: bool,
    _velocity: Force,
}

impl Particle{
    pub fn new(_id: usize) -> Particle {
        Particle { 
            _id, 
            _updated: false, 
            _deleted: false,
            _velocity: Force { _x: 0.0, _y: 0.0 }
        }
    }

    pub fn empty() -> Particle {
        Particle {
        _id: usize::MAX,
        _updated: false,
        _deleted: false,
        _velocity: Force { _x: 0.0, _y: 0.0 }
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

    pub fn set_velocity(&mut self) -> &mut Force{
        &mut self._velocity
    }

    pub fn velocity(&self) -> Force {
        self._velocity
    }

    pub fn update(&mut self, value: bool){
        self._updated = value;
    }

    pub fn delete(&mut self, value: bool){
        self._deleted = value;
    }
}
