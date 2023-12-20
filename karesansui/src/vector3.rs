pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        Vector3 {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn magnitude(&self) -> f64 {
        let power = 2_f64;

        (self.x.powf(power) + self.y.powf(power) + self.z.powf(power)).sqrt()
    }

    pub fn squared_magnitude(&self) -> f64 {
        let power = 2_f64;

        self.x.powf(power) + self.y.powf(power) + self.z.powf(power)
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        if magnitude <= 0_f64 {
            return;
        }

        self.scale(1_f64 / magnitude);
    }

    pub fn scale(&mut self, scaler: f64) {
        self.x *= scaler;
        self.y *= scaler;
        self.z *= scaler;
    }

    pub fn create_scaled(&self, scaler: f64) -> Vector3 {
        Vector3 {
            x: self.x * scaler,
            y: self.y * scaler,
            z: self.z * scaler,
        }
    }

    pub fn add(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn create_added(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&mut self, other: &Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    pub fn create_subtracted(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn into_component_product(&mut self, other: &Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.y;
    }

    pub fn create_component_product(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// dot product
    pub fn scalar_product(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// cross product
    pub fn into_vector_product(&mut self, other: &Vector3) {
        self.x = self.y * other.z - self.z * other.y;
        self.y = self.z * other.x - self.x * other.z;
        self.z = self.x * other.y - self.y * other.x;
    }

    /// cross product
    pub fn create_vector_product(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
