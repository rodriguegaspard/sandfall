// Definition of the Element class, which defines a Particle's behaviour.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Element{
    _name : String,
    _description : String
}

// In the future, add categories (organics, liquids, solids, gases..)
pub struct ElementTable{
    _table : [Element; 5]
}

impl Default for ElementTable{
    fn default() -> Self {
        ElementTable { 
            _table:
                [
                Element {
                    _name: String::from("Water"),
                    _description: String::from("Extinguishes fires, freezes into ice below 0C and turns into steam when above 100C.")
                },
                Element {
                    _name: String::from("Fire"),
                    _description: String::from("Heats up elements, and make some combusts. Releases smoke.")
                },
                Element {
                    _name: String::from("Dirt"),
                    _description: String::from("Transforms into mud when in contact with water. Allows plants to grow on it.")
                },
                Element {
                    _name: String::from("Stone"),
                    _description: String::from("Extinguishes fires, freezes into ice below 0C and turns into steam when above 100C.")
                },
                Element {
                    _name: String::from("Oil"),
                    _description: String::from("Floats on top of water and other, less dense liquids. Combusts when in contact of fire.")
                },
                ] 
        }
    }
}
 
impl ElementTable{
    pub fn name(&self, id: usize) -> &String{
        &self._table[id]._name
    }

    pub fn description(&self, id: usize) -> &String{
        &self._table[id]._description
    }
} 
