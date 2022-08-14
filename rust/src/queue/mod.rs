//! Queue Rust interface implementation.
pub mod queue;

trait MsgQueue {
    /// Create a new event queue with the information
    /// provided as parameter.
    fn new(backend: &Box<dyn Backed>) -> Self;
}

trait Backed {
    /// Create a new backend with the information provided
    /// TODO: how pass variable information as parameter?
    fn new() -> Self
    where
        Self: Sized;
}
