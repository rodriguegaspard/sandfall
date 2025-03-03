use wasm_bindgen::prelude::*;
use crate::{element::ElementTable, Particle};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Square{
    _x: f64,
    _y: f64,
    _unit: f64
}

impl Square{
    pub fn new(x: f64, y: f64, unit: f64) -> Self{
        Square{
            _x: x,
            _y: y,
            _unit: unit,
        }
    }
    pub fn x(&self) -> f64{
        self._x
    }

    pub fn y(&self) -> f64{
        self._y
    }   

    pub fn unit(&self) -> f64{
        self._unit
    }
}

#[wasm_bindgen]
pub struct ParticleWorld{
    _particle: Option<Particle>,
    _dimensions: Square, 
    _quadrants: [Option<Box<ParticleWorld>>; 4], // northwest, northeast, southwest, southeast 
}

impl ParticleWorld {
    pub fn new(dimensions: Square) -> Self {
        ParticleWorld {
            _particle: None,
            _dimensions: dimensions,
            _quadrants: [None, None, None, None],
        }
    }

    pub fn print_particle(&self, elements: &ElementTable) -> String {
        match &self._particle{
            Some(particle) => format!("This particle is made of {} and is at ({};{})" , particle.element(elements), &self._dimensions._x, &self._dimensions._y),
            None => "Nothing!".to_string(),
        }
    }

    pub fn print_bounds(&self) -> String {
        format!("This quadrant had its origin at ({},{}) and is of unit {}", &self._dimensions._x, &self._dimensions._y, &self._dimensions._unit)
    }

    pub fn split_tree(&mut self) {
        if self.is_leaf(){
            let half_unit = self._dimensions._unit / 2.0;
            self._quadrants[0] = Some(Box::new(ParticleWorld::new(Square::new(self._dimensions._x, self._dimensions._y, half_unit))));
            self._quadrants[1] = Some(Box::new(ParticleWorld::new(Square::new(self._dimensions._x + half_unit, self._dimensions._y, half_unit))));
            self._quadrants[2] = Some(Box::new(ParticleWorld::new(Square::new(self._dimensions._x, self._dimensions._y + half_unit, self._dimensions._unit))));
            self._quadrants[3] = Some(Box::new(ParticleWorld::new(Square::new(self._dimensions._x + half_unit, self._dimensions._y + half_unit, self._dimensions._unit))));
        }
    }

    pub fn is_leaf(&self) -> bool {
        self._quadrants[0].is_none()
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        (self._dimensions._x + self._dimensions._unit >= x) && (self._dimensions._y + self._dimensions._unit >= y)  
    }

    pub fn is_at_max_depth(&self) -> bool {
        self._dimensions._unit <= 1.0
    }

    pub fn insert(&mut self, particle: Particle, x: f64, y: f64) -> bool {
        if ! self.contains(x, y){
            false;
        }
        else 
            if self.is_at_max_depth(){
                if self._particle.is_none(){
                    self._particle = Some(particle);
                    true;
                }
                else {
                    false;
                }
            }
            else 
                if self.is_leaf(){
                    self.split_tree();
                    for child in &mut self._quadrants{
                        if child.as_ref().unwrap().contains(x, y){
                            return self.insert(particle, x, y);
                        }
                    }
                }
                else{
                    for child in &mut self._quadrants{
                        if child.as_ref().unwrap().contains(x, y){
                            return child.as_mut().unwrap().insert(particle, x, y);
                        }
                    }
                }
        false
    }

    pub fn get_quadrants(&self, data: &mut Vec<Square>){
        if self.is_leaf(){
            data.push(self._dimensions);
        }
        else {
            for child in &self._quadrants{
                child.as_ref().unwrap().get_quadrants(data);
            }
        }
    }

    pub fn get_particles(&self, data: &mut Vec<Square>){
        if self.is_leaf(){
            if self.is_at_max_depth() && self._particle.is_some(){
                data.push(self._dimensions);
            }
        }
        else {
            for child in &self._quadrants{
                child.as_ref().unwrap().get_particles(data);
            }
        }
    }


    pub fn search(&mut self, x: f64, y: f64) -> Option<&mut Particle> {
        if self.is_at_max_depth(){
            self._particle.as_mut()
        }
        else{
            for child in &mut self._quadrants {
                if child.as_ref().unwrap().contains(x, y){
                    return child.as_mut().unwrap().search(x, y);
                }
            }
            None
        }
    }

    pub fn delete(&mut self, x: f64, y: f64) {
        if self.is_at_max_depth() && self.contains(x, y){
            self._particle = None;
        }
        else {
            for child in &mut self._quadrants {
                if child.as_mut().unwrap().contains(x, y){
                    child.as_mut().unwrap().delete(x, y);
                }
            }
        }
    }

    pub fn balance(&mut self) -> bool {
        if self.is_at_max_depth() && self._particle.is_none(){
            true;
        }
        for child in &mut self._quadrants {
            if child.as_mut().unwrap().balance(){
                false;
            }
        }
        false
    }
}
