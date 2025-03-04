#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod permutations;
pub mod keygen;
pub mod encryption;
mod utils;
mod aont;

pub use encryption::*;
pub use keygen::*;

const BLOCK_SIZE_BITS: usize = 32;
const BLOCK_SIZE_BYTE: usize = BLOCK_SIZE_BITS >> 3;
const MESSAGE_BLOCKS_COUNT: usize = 8;