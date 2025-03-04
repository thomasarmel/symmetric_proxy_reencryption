use collar::CollectArray;
use rand::Rng;
use sha3::{Digest, Sha3_256};
use crate::{BLOCK_SIZE_BYTE, MESSAGE_BLOCKS_COUNT};
use crate::utils::xor_array;

#[derive(Debug, Clone)]
pub(crate) struct AONT  {
    pub(crate) encrypted: [u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE]
}

impl AONT {
    pub(crate) fn new(aont_encrypted_bytes: &[u8; (MESSAGE_BLOCKS_COUNT + 1) * BLOCK_SIZE_BYTE]) -> Self {
        Self {
            encrypted: *aont_encrypted_bytes
        }
    }

    pub(crate) fn from_message(input: &[u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE]) -> Self {
        let mut rng = rand::thread_rng();
        let mut hasher = Sha3_256::new();
        let mut random_key_xor = [0u8; BLOCK_SIZE_BYTE];
        rng.fill(&mut random_key_xor);
        let xored: [u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE] = input.chunks_exact(BLOCK_SIZE_BYTE).map(|chunk| xor_array::<4>(chunk.try_into().unwrap(), &random_key_xor)).flatten().collect_array();
        Digest::update(&mut hasher, &xored);
        let hash: [u8; BLOCK_SIZE_BYTE] = (&hasher.finalize()[..BLOCK_SIZE_BYTE]).try_into().unwrap();
        let hash_xor_key = xor_array(&hash, &random_key_xor);
        Self {
            encrypted: xored.iter().chain(hash_xor_key.iter()).cloned().collect_array()
        }
    }

    pub(crate) fn retrieve_message(&self) -> [u8; MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE] {
        let mut hasher = Sha3_256::new();
        let (message, hash_xor_key) = self.encrypted.split_at(MESSAGE_BLOCKS_COUNT * BLOCK_SIZE_BYTE);
        Digest::update(&mut hasher, message);
        let hash: [u8; BLOCK_SIZE_BYTE] = (&hasher.finalize()[..BLOCK_SIZE_BYTE]).try_into().unwrap();
        let random_key_xor = xor_array(&hash, &hash_xor_key.try_into().unwrap());
        message.chunks_exact(BLOCK_SIZE_BYTE).map(|chunk| xor_array::<4>(chunk.try_into().unwrap(), &random_key_xor)).flatten().collect_array()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_aont() {
        let message = [0u8; 32];
        let aont = super::AONT::from_message(&message);
        let retrieved_message = aont.retrieve_message();
        assert_eq!(message, retrieved_message);
    }
}