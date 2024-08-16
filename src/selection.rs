use std::collections::HashSet;
use super::*;

#[derive(Default)]
pub struct Selection {
    pub vertices: HashSet<ArcPlus<Vertex>>,
    pub edges: HashSet<ArcPlus<Edge>>,
    pub faces: HashSet<ArcPlus<Face>>,
}

impl Selection {
    
}