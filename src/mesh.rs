use std::fs;
use regex::{Captures, Regex};
use super::*;

mod import;
mod export;

/// Mesh
#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }
}