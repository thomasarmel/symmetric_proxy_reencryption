use crate::{BLOCK_SIZE_BITS, BLOCK_SIZE_BYTE, MESSAGE_BLOCKS_COUNT};
use factorial::Factorial;
use num_bigint::{BigUint, RandBigInt};
use num_traits::Zero;
use rand::Rng;
use crate::permutations::{find_permute_conversion, generate_permutation, get_permutation_number};

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

pub struct ReEncryptionKey {
    ck1: BigUint,
    ck3: BigUint,
    old_kx: [u8; BLOCK_SIZE_BYTE],
    new_kx: [u8; BLOCK_SIZE_BYTE],
    old_k2: BigUint,
    new_k2: BigUint,
}

impl ReEncryptionKey {
    pub fn generate(old_key: &Key, new_key: &Key) -> Self {
        let p1_old = old_key.p1();
        let p3_old = old_key.p3();
        let p1_new = new_key.p1();
        let p3_new = new_key.p3();
        let ck1 = get_permutation_number(find_permute_conversion(&p1_old, &p1_new));
        let ck3 = get_permutation_number(find_permute_conversion(&p3_old, &p3_new));

        Self {
            ck1,
            ck3,
            old_kx: old_key.kx(),
            new_kx: new_key.kx(),
            old_k2: old_key.k2.clone(),
            new_k2: new_key.k2.clone(),
        }
    }

    pub(crate) fn old_p2(&self) -> [usize; BLOCK_SIZE_BITS] {
        generate_permutation::<BLOCK_SIZE_BITS>(self.old_k2.clone())
    }

    pub(crate) fn new_p2(&self) -> [usize; BLOCK_SIZE_BITS] {
        generate_permutation::<BLOCK_SIZE_BITS>(self.new_k2.clone())
    }

    pub(crate) fn cp1(&self) -> [usize; BLOCK_SIZE_BITS] {
        generate_permutation(self.ck1.clone())
    }

    pub(crate) fn cp3(&self) -> [usize; MESSAGE_BLOCKS_COUNT + 1] {
        generate_permutation(self.ck3.clone())
    }

    pub(crate) fn old_kx(&self) -> [u8; BLOCK_SIZE_BYTE] {
        self.old_kx
    }

    pub(crate) fn new_kx(&self) -> [u8; BLOCK_SIZE_BYTE] {
        self.new_kx
    }
}