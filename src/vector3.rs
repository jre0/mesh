use std::ops::Sub;

pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    pub fn new(vector: &[f64]) -> Self {
        Self {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
    pub fn as_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}