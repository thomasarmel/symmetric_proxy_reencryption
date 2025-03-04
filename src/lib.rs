#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![doc = include_str!("../README.md")]

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

pub type Encrypted = [u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE];
pub type Plaintext = [u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE];