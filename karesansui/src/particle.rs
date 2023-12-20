use crate::vector3::Vector3;

pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    inverse_mass: f64,

    // Required to remove energy added through numerical instability in the integrator
    // Drag
    dumping: f64,
}

impl Particle {
    pub fn set_mass(&mut self, value: f64) {
        self.inverse_mass = 1_f64 / value;
    }

    pub fn set_inverse_mass(&mut self, value: f64) {
        self.inverse_mass = value;
    }
}
