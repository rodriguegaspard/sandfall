use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Element{
    _name : &'static str,
    _description : &'static str
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ElementTable{
    _table : [Element; 5]
}

impl Default for ElementTable{
    fn default() -> Self {
        ElementTable { 
            _table:
                [
                Element {
                    _name: "Water",
                    _description: "Extinguishes fires, freezes into ice below 0C and turns into steam when above 100C."
                },
                Element {
                    _name: "Fire",
                    _description: "Heats up elements, and make some combusts. Releases smoke."
                },
                Element {
                    _name: "Dirt",
                    _description: "Transforms into mud when in contact with water. Allows plants to grow on it."
                },
                Element {
                    _name: "Stone",
                    _description: "Extinguishes fires, freezes into ice below 0C and turns into steam when above 100C."
                },
                Element {
                    _name: "Oil",
                    _description: "Floats on top of water and other, less dense liquids. Combusts when in contact of fire."
                },
                ] 
        }
    }
}
 
impl ElementTable{
    pub fn name(&self, id: usize) -> &str{
        &self._table[id]._name
    }

    pub fn description(&self, id: usize) -> &str{
        &self._table[id]._description
    }
} 
