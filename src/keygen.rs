use crate::{BLOCK_SIZE_BITS, BLOCK_SIZE_BYTE, MESSAGE_BLOCKS_COUNT};
use factorial::Factorial;
use num_bigint::{BigUint, RandBigInt};
use num_traits::Zero;
use rand::Rng;
use crate::permutations::generate_permutation;

#[derive(Debug, Clone)]
pub struct Key {
    k1: BigUint,
    k2: BigUint,
    k3: BigUint,
    kx: [u8; BLOCK_SIZE_BYTE],
}

impl Key {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let max_value_block_perm = BigUint::from(BLOCK_SIZE_BITS).factorial();
        let max_value_block_count = BigUint::from(MESSAGE_BLOCKS_COUNT + 1).factorial();
        // generate a random number between 0 and max_value
        let k1 = rng.gen_biguint_range(&BigUint::zero(), &max_value_block_perm);
        let k2 = rng.gen_biguint_range(&BigUint::zero(), &max_value_block_perm);
        let k3 = rng.gen_biguint_range(&BigUint::zero(), &max_value_block_count);
        let mut kx = [0u8; BLOCK_SIZE_BYTE];
        rng.fill(&mut kx);
        Self { k1, k2, k3, kx }
    }

    pub(crate) fn p1(&self) -> [usize; BLOCK_SIZE_BITS] {
        generate_permutation::<BLOCK_SIZE_BITS>(self.k1.clone())
    }

    pub(crate) fn p2(&self) -> [usize; BLOCK_SIZE_BITS] {
        generate_permutation::<BLOCK_SIZE_BITS>(self.k2.clone())
    }

    pub(crate) fn p3(&self) -> [usize; MESSAGE_BLOCKS_COUNT + 1] {
        generate_permutation::<{ MESSAGE_BLOCKS_COUNT + 1 }>(self.k3.clone())
    }

    pub(crate) fn kx(&self) -> [u8; BLOCK_SIZE_BYTE] {
        self.kx
    }
}