use super::*;

#[derive(Default, PartialEq, Eq, Hash, Debug)]
pub struct Edge {
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
}