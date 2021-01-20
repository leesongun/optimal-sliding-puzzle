//#![no_std]
mod hasher;
mod markov;
pub mod search;

pub use markov::instances::*;
pub use markov::Action;
pub use markov::State;

pub use hasher::Builder;

//todo :
//additive pdb
//residual pdb
//bidirectional A*
//gitlab ci
