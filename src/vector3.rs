use std::ops::Sub;
use super::*;

/// 3D vector to represent coordinates and directions
#[derive(Default, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(vector: [f64; 3]) -> Self {
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
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn normalized(&self) -> Result<Self, Error> {
        let length = self.length();
        if length > 0.00001 {
            Ok(Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            })
        } else {
            Err("Vector3 too short to normalize.")?
        }
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


// impl PartialEq for Vector3 {
//     fn eq(&self, other: &Self) -> bool {
//         (self - other).length() < EP
//     }
// }

// impl Hash for Vector3 {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.x.to_bits().hash(state);
//         self.y.to_bits().hash(state);
//         self.z.to_bits().hash(state);
//     }
// }