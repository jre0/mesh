pub use vector3::*;
pub use config::*;
pub use mesh::*;
pub use face::*;
pub use edge::*;
pub use vertex::*;
pub use selection::*;

use std::{error, hash::Hash, sync::{Arc, Weak}};

#[cfg(test)]
mod tests;
mod mesh;
mod vector3;
mod config;
mod face;
mod edge;
mod vertex;
mod selection;

pub type Error = Box<dyn error::Error>;

pub struct ArcPlus<T>(Arc<T>);

impl<T: Default> Default for ArcPlus<T> {
    fn default() -> Self {
        Self(Arc::new(T::default()))
    }
}

impl<T> Clone for ArcPlus<T> {
    /// Clone the pointer.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Eq for ArcPlus<T> where T: Eq {}

impl<T> PartialEq for ArcPlus<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Hash> Hash for ArcPlus<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> ArcPlus<T> {
    pub fn new(item: Arc<T>) -> Self {
        Self(item)   
    }
}

// pub struct WeakPlus<T>(Weak<T>);

// impl<T> Eq for WeakPlus<T> where T: Eq {}

// impl<T> PartialEq for WeakPlus<T> {
//     fn eq(&self, other: &Self) -> bool {
//         Weak::ptr_eq(&self.0, &other.0)
//     }
// }

// impl<T: Hash> Hash for WeakPlus<T> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         //self.0.hash(state);
//     }
// }