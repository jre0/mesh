use std::error;

pub use vector3::*;
pub use config::*;
pub use mesh::*;

#[cfg(test)]
mod tests;
mod mesh;
mod vector3;
mod config;

pub type Error = Box<dyn error::Error>;