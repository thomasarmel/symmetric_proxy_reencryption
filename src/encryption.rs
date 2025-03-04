use crate::aont::AONT;
use crate::keygen::Key;
use crate::permutations::{depermute_block, depermute_block_set, permute_block, permute_block_set};
use crate::utils::xor_array;
use crate::{Encrypted, Plaintext, ReEncryptionKey, BLOCK_SIZE_BITS, BLOCK_SIZE_BYTE, MESSAGE_BLOCKS_COUNT};
use collar::CollectArray;

pub fn encrypt(input: &Plaintext, key: &Key) -> Encrypted {
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

pub fn decrypt(encrypted: &Encrypted, key: &Key) -> Plaintext {
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

pub fn re_encrypt(encrypted: &Encrypted, reencryption_key: &ReEncryptionKey) -> Encrypted {
    let old_p2 = reencryption_key.old_p2();
    let new_p2 = reencryption_key.new_p2();
    let cp1 = reencryption_key.cp1();
    let cp3 = reencryption_key.cp3();
    let mut reperm_encrypted_blocks = [0u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE];
    let c0: [u8; BLOCK_SIZE_BYTE] = encrypted[0..BLOCK_SIZE_BYTE].try_into().unwrap();
    let c0_prime: [u8; BLOCK_SIZE_BYTE] = permute_block(&xor_array(&c0, &permute_block(&reencryption_key.old_kx(), &old_p2)), &cp1);
    reperm_encrypted_blocks[0..BLOCK_SIZE_BYTE].copy_from_slice(&c0_prime);
    for i in 1..=MESSAGE_BLOCKS_COUNT {
        let previous_ci: [u8; BLOCK_SIZE_BYTE] = encrypted[(i - 1) * BLOCK_SIZE_BYTE..i * BLOCK_SIZE_BYTE].try_into().unwrap();
        let ci: [u8; BLOCK_SIZE_BYTE] = encrypted[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].try_into().unwrap();
        let ci_prime: [u8; BLOCK_SIZE_BYTE] = permute_block(&xor_array(&ci, &permute_block(&previous_ci, &old_p2)), &cp1);
        reperm_encrypted_blocks[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].copy_from_slice(&ci_prime);
    }

    let splitted_reperm_encrypted_blocks: [&[u8; BLOCK_SIZE_BYTE]; MESSAGE_BLOCKS_COUNT + 1] = reperm_encrypted_blocks
        .chunks_exact(BLOCK_SIZE_BYTE)
        .map(|block| block.try_into().unwrap())
        .collect_array();
    let correct_permuted_blocks: [u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE] = permute_block_set(&splitted_reperm_encrypted_blocks, &cp3)
        .map(|block| *block)
        .iter()
        .flatten()
        .map(|b| *b)
        .collect_array();

    let mut output = [0u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE];
    let c0: [u8; BLOCK_SIZE_BYTE] = correct_permuted_blocks[0..BLOCK_SIZE_BYTE].try_into().unwrap();
    let new_c0: [u8; BLOCK_SIZE_BYTE] = xor_array(&c0, &permute_block(&reencryption_key.new_kx(), &new_p2));
    output[0..BLOCK_SIZE_BYTE].copy_from_slice(&new_c0);
    for i in 1..=MESSAGE_BLOCKS_COUNT {
        let previous_ci: [u8; BLOCK_SIZE_BYTE] = output[(i - 1) * BLOCK_SIZE_BYTE..i * BLOCK_SIZE_BYTE].try_into().unwrap();
        let ci: [u8; BLOCK_SIZE_BYTE] = correct_permuted_blocks[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].try_into().unwrap();
        let new_ci: [u8; BLOCK_SIZE_BYTE] = xor_array(&ci, &permute_block(&previous_ci, &new_p2));
        output[i * BLOCK_SIZE_BYTE..(i + 1) * BLOCK_SIZE_BYTE].copy_from_slice(&new_ci);
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt, re_encrypt, Key, ReEncryptionKey};

    #[test]
    fn test_encryption_reencryption() {
        let message = b"les sanglots longs des violons !";
        let key1 = Key::generate();
        let encrypted = encrypt(message, &key1);
        let decrypted = decrypt(&encrypted, &key1);
        assert_eq!(message, &decrypted);

        let key2 = Key::generate();
        let re_encryption_key = ReEncryptionKey::generate(&key1, &key2);
        let re_encrypted = re_encrypt(&encrypted, &re_encryption_key);
        let re_decrypted = decrypt(&re_encrypted, &key2);
        assert_eq!(message, &re_decrypted);
    }
}