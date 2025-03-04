use crate::aont::AONT;
use crate::keygen::Key;
use crate::permutations::{depermute_block, depermute_block_set, permute_block, permute_block_set};
use crate::utils::xor_array;
use crate::{BLOCK_SIZE_BITS, BLOCK_SIZE_BYTE, MESSAGE_BLOCKS_COUNT};
use collar::CollectArray;

pub fn encrypt(input: &[u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE], key: &Key) -> [u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE] {
    let aont = AONT::from_message(input);
    let p1 = key.p1();
    let p2 = key.p2();
    let p3 = key.p3();
    let aont_blocks: [&[u8; BLOCK_SIZE_BYTE]; MESSAGE_BLOCKS_COUNT + 1] = aont.encrypted.chunks_exact(BLOCK_SIZE_BYTE).map(|block| block.try_into().unwrap()).collect_array();
    let aont_permuted_blocks = permute_block_set(&aont_blocks, &p3);
    let mut output = [0u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE];
    let c0: [u8; BLOCK_SIZE_BYTE] = xor_array::<BLOCK_SIZE_BYTE>(&permute_block::<BLOCK_SIZE_BITS>(aont_permuted_blocks[0], &p1), &permute_block::<BLOCK_SIZE_BITS>(&key.kx(), &p2));
    output[0..BLOCK_SIZE_BYTE].copy_from_slice(&c0);
    for i in 1..=MESSAGE_BLOCKS_COUNT {
        let previous_encrypted_block: &[u8; BLOCK_SIZE_BYTE] = &output[(i-1) * BLOCK_SIZE_BYTE..i * BLOCK_SIZE_BYTE].try_into().unwrap();
        let ci: [u8; BLOCK_SIZE_BYTE] = xor_array::<BLOCK_SIZE_BYTE>(&permute_block::<BLOCK_SIZE_BITS>(aont_permuted_blocks[i], &p1),
                                                                     &permute_block::<BLOCK_SIZE_BITS>(previous_encrypted_block, &p2));
        output[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].copy_from_slice(&ci);
    }
    output
}

pub fn decrypt(encrypted: &[u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE], key: &Key) -> [u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE] {
    let p1 = key.p1();
    let p2 = key.p2();
    let p3 = key.p3();
    let mut permuted_aont = [0u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE];
    let mp0: [u8; BLOCK_SIZE_BYTE] = depermute_block::<BLOCK_SIZE_BITS>(&xor_array::<BLOCK_SIZE_BYTE>(encrypted[0..BLOCK_SIZE_BYTE].try_into().unwrap(), &permute_block::<BLOCK_SIZE_BITS>(&key.kx(), &p2)), &p1);
    permuted_aont[0..BLOCK_SIZE_BYTE].copy_from_slice(&mp0);
    for i in 1..=MESSAGE_BLOCKS_COUNT {
        let previous_encrypted_block: &[u8; BLOCK_SIZE_BYTE] = &encrypted[(i-1) * BLOCK_SIZE_BYTE..i * BLOCK_SIZE_BYTE].try_into().unwrap();
        let ci: [u8; BLOCK_SIZE_BYTE] = encrypted[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].try_into().unwrap();
        let mpi: [u8; BLOCK_SIZE_BYTE] = depermute_block::<BLOCK_SIZE_BITS>(&xor_array::<BLOCK_SIZE_BYTE>(&ci, &permute_block::<BLOCK_SIZE_BITS>(previous_encrypted_block, &p2)), &p1);
        permuted_aont[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].copy_from_slice(&mpi);
    }
    let permuted_aont: [&[u8; BLOCK_SIZE_BYTE]; MESSAGE_BLOCKS_COUNT + 1] = permuted_aont.chunks_exact(BLOCK_SIZE_BYTE).map(|block| block.try_into().unwrap()).collect_array();
    let aont_bytes: [u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE] = depermute_block_set(&permuted_aont, &p3).map(|block| *block).iter().flatten().map(|b| *b).collect_array();
    let aont = AONT::new(&aont_bytes);
    aont.retrieve_message()
}