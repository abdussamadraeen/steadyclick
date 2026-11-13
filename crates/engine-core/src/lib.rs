//! SteadyClick Engine Core

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod kinematics;
pub mod linguistics;
pub mod perceptual;
pub mod wasm_bindings;

pub use kinematics::*;
pub use linguistics::*;
pub use perceptual::*;
