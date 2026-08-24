//! Integrate Flutter with Rust

mod creator;
pub(crate) mod integrator;
mod utils;

pub use creator::{CreateConfig, create};
pub use integrator::{IntegrateConfig, integrate};
