use std::collections::HashSet;
use super::*;

#[derive(Default)]
pub struct Selection {
    pub vertices: HashSet<Pointer<Vertex>>,
    pub edges: HashSet<Pointer<Edge>>,
    pub faces: HashSet<Pointer<Face>>,
}

impl Selection {
    
}