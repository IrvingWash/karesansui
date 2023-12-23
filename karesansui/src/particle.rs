use crate::vector3::Vector3;

pub struct Particle {
    // Pub for debugging
    pub position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    inverse_mass: f64,

    // Required to remove energy added through numerical instability in the integrator
    // Drag
    damping: f64,
}

impl Particle {
    pub fn zero() -> Self {
        Particle {
            position: Vector3::zero(),
            velocity: Vector3::zero(),
            acceleration: Vector3::zero(),
            inverse_mass: 1_f64,
            damping: 0.999,
        }
    }

    // Debug method
    pub fn set_velocity(&mut self, velocity: Vector3) {
        self.velocity = velocity;
    }

    pub fn set_mass(&mut self, value: f64) {
        self.inverse_mass = 1_f64 / value;
    }

    pub fn set_inverse_mass(&mut self, value: f64) {
        self.inverse_mass = value;
    }

    pub fn integrate(&mut self, duration: f64) {
        // Don't need to integrate particles with infinite mass
        if self.inverse_mass <= 0_f64 {
            return;
        }

        // Update linear position
        self.position.add(&self.velocity.create_scaled(duration));

        // Update linear velocity from the acceleration
        self.velocity.add(&self.acceleration.create_scaled(duration));

        // Impose drag
        self.velocity.scale(self.damping.powf(duration));

        // Clear the forces
        self.clear_accumulator();
    }

    fn clear_accumulator(&self) {
        return;
    }
}
