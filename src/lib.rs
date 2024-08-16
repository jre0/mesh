pub use vector3::*;
pub use config::*;
pub use mesh::*;
pub use face::*;
pub use edge::*;
pub use vertex::*;
pub use pick::*;

use std::error;

#[cfg(test)]
mod tests;
mod mesh;
mod vector3;
mod config;
mod face;
mod edge;
mod vertex;
mod pick;

pub type Error = Box<dyn error::Error>;