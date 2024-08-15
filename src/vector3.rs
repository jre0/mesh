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
}