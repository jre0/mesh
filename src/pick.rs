use std::{collections::HashSet, sync::Arc};
use super::*;

#[derive(Default)]
pub struct Pick {
    pub vertices: HashSet<Arc<Vertex>>,
    pub edges: HashSet<Arc<Edge>>,
    pub faces: HashSet<ArcFace>,
}

impl Pick {
    // pub fn add_faces(&mut self, faces)
}