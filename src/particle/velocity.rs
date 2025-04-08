use super::Particle;

#[derive(Copy, Clone, Debug)]
pub struct Force {
    pub _x: f32,
    pub _y: f32,
}

impl Force {
    pub fn new(_x: f32, _y: f32) -> Self {
        Force { _x, _y }
    }

    pub fn x(&self) -> f32 {
        self._x
    }

    pub fn y(&self) -> f32 {
        self._y
    }

    pub fn add(&mut self, force: Force) {
        self._x += force._x;
        self._y += force._y;
    }

    pub fn scale(&mut self, scalar: f32) {
        self._x *= scalar;
        self._y *= scalar;
    }

    pub fn magnitude(&self) -> f32 {
        (self._x * self._x + self._y * self._y).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.magnitude();
        if len > 0.0 {
            self._x /= len;
            self._y /= len;
        }
    }

}

pub trait Gravity{
    fn apply_gravity(&mut self);
}

impl Gravity for Particle{
    fn apply_gravity(&mut self){
        self.set_velocity().add(Force::new(0.0, 0.0098));
    }
}
