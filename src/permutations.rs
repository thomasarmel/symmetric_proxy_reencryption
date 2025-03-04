use collar::CollectArray;
use num_bigint::BigUint;
use num_traits::Zero;

pub(crate) fn permute_block<const BIT_COUNT: usize>(input: &[u8; BIT_COUNT >> 3], permutation_key: &[usize; BIT_COUNT]) -> [u8; BIT_COUNT >> 3] {
    let bit_count = input.len() << 3;
    let mut output = [0u8; BIT_COUNT >> 3];
    for i in 0..bit_count {
        let next_pos = permutation_key[i];
        assert!(next_pos < bit_count);
        let input_byte_pos = next_pos >> 3;
        let input_bit_pos = 7 - (next_pos & 7);
        let input_bit = (input[input_byte_pos] >> input_bit_pos) & 1;
        let output_byte_pos = i >> 3;
        let output_bit_pos = 7 - (i & 7);
        output[output_byte_pos] |= input_bit << output_bit_pos;
    }
    output
}

pub(crate) fn depermute_block<const BIT_COUNT: usize>(input: &[u8; BIT_COUNT >> 3], permutation_key: &[usize; BIT_COUNT]) -> [u8; BIT_COUNT >> 3] {
    let bit_count = input.len() << 3;
    let mut output = [0u8; BIT_COUNT >> 3];
    for i in 0..bit_count {
        let next_pos = permutation_key[i];
        assert!(next_pos < bit_count);
        let input_byte_pos = i >> 3;
        let input_bit_pos = 7 - (i & 7);
        let input_bit = (input[input_byte_pos] >> input_bit_pos) & 1;
        let output_byte_pos = next_pos >> 3;
        let output_bit_pos = 7 - (next_pos & 7);
        output[output_byte_pos] |= input_bit << output_bit_pos;
    }
    output
}

pub(crate) fn permute_block_set<'a, const BLOCKS_COUNT: usize, const BYTES_PER_BLOCK: usize>(input_blocks: &[&'a [u8; BYTES_PER_BLOCK]; BLOCKS_COUNT], permutation_key: &[usize; BLOCKS_COUNT]) -> [&'a [u8; BYTES_PER_BLOCK]; BLOCKS_COUNT] {
    let blocks_count = input_blocks.len();
    let mut output = *input_blocks;
    for i in 0..blocks_count {
        let next_pos = permutation_key[i];
        assert!(next_pos < blocks_count);
        output[i] = input_blocks[next_pos];
    }
    output
}

pub(crate) fn depermute_block_set<'a, const BLOCKS_COUNT: usize, const BYTES_PER_BLOCK: usize>(input_blocks: &[&'a [u8; BYTES_PER_BLOCK]; BLOCKS_COUNT], permutation_key: &[usize; BLOCKS_COUNT]) -> [&'a [u8; BYTES_PER_BLOCK]; BLOCKS_COUNT] {
    let blocks_count = input_blocks.len();
    let mut output = *input_blocks;
    for i in 0..blocks_count {
        let next_pos = permutation_key[i];
        assert!(next_pos < blocks_count);
        output[next_pos] = input_blocks[i];
    }
    output
}

pub(crate) fn find_permute_conversion<const BIT_COUNT: usize>(old_permutation: &[usize; BIT_COUNT], new_permutation: &[usize; BIT_COUNT]) -> [usize; BIT_COUNT] {
    let mut conversion = [0; BIT_COUNT];
    for i in 0..BIT_COUNT {
        for j in 0..BIT_COUNT {
            if old_permutation[i] == new_permutation[j] {
                conversion[j] = i;
                break;
            }
        }
    }
    conversion
}

pub(crate) fn generate_permutation<const ELEMENTS_COUNT: usize>(mut n: BigUint) -> [usize; ELEMENTS_COUNT] {
    let mut stack: Vec<usize> = Vec::new();
    let mut result = [0usize; ELEMENTS_COUNT];
    let mut input_elements: [usize; ELEMENTS_COUNT] = (0..ELEMENTS_COUNT).collect_array();

    for i in 1..=ELEMENTS_COUNT {
        stack.push((n.clone() % i).try_into().unwrap());
        n = n / i;
    }

    for i in 0..ELEMENTS_COUNT {
        let a = stack.pop().unwrap();
        result[i] = input_elements[a];
        for j in a..ELEMENTS_COUNT - 1 {
            input_elements[j] = input_elements[j + 1];
        }
    }
    result
}

pub(crate) fn get_permutation_number<const ELEMENTS_COUNT: usize>(permutation: [usize; ELEMENTS_COUNT]) -> BigUint {
    let mut input_elements: Vec<usize> = (0..ELEMENTS_COUNT).collect();
    let mut n = BigUint::zero();

    for i in 0..ELEMENTS_COUNT {
        let a = input_elements.iter().position(|&x| x == permutation[i]).unwrap();
        n = n * (ELEMENTS_COUNT - i) + BigUint::from(a);
        input_elements.remove(a);
    }

    n
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::Zero;

    #[test]
    fn test_permute_block() {
        let input = [0b0000_0001, 0b0000_0010];
        let permutation_key = [0, 1, 2, 3, 4, 5, 7, 6, 8, 9, 10, 11, 12, 13, 15, 14];
        let output = super::permute_block(&input, &permutation_key);
        assert_eq!(output, [0b0000_0010, 0b0000_0001]);
    }

    #[test]
    fn test_depermute_block() {
        let input = [0b0000_0010, 0b0000_0001];
        let permutation_key = [0, 1, 2, 3, 4, 5, 7, 6, 8, 9, 10, 11, 12, 13, 15, 14];
        let output = super::depermute_block(&input, &permutation_key);
        assert_eq!(output, [0b0000_0001, 0b0000_0010]);
    }

    #[test]
    fn test_find_permute_conversion() {
        let old_permutation = [0, 1, 2, 3, 4, 5, 7, 6, 8, 9, 10, 11, 12, 13, 15, 14];
        let new_permutation = [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let conversion = super::find_permute_conversion(&old_permutation, &new_permutation);
        assert_eq!(conversion, [1, 0, 2, 3, 4, 5, 7, 6, 8, 9, 10, 11, 12, 13, 15, 14]);
    }

    #[test]
    fn test_generate_permutation() {
        let no_perm = super::generate_permutation::<16>(0usize.into());
        assert_eq!(no_perm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let perm = super::generate_permutation::<16>(1usize.into());
        assert_eq!(perm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14]);
        let perm = super::generate_permutation::<16>(2usize.into());
        assert_eq!(perm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 13, 15]);
    }

    #[test]
    fn test_get_permutation_number() {
        let perm = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let n = super::get_permutation_number(perm);
        assert_eq!(n, BigUint::zero());
        let perm = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14];
        let n = super::get_permutation_number(perm);
        assert_eq!(n, 1usize.into());
        let perm = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 13, 15];
        let n = super::get_permutation_number(perm);
        assert_eq!(n, 2usize.into());
    }

    #[test]
    fn test_permute_block_set() {
        let blocks = [
            &[0b0000_0001, 0b0000_0010],
            &[0b0000_0011, 0b0000_0100],
            &[0b0000_0101, 0b0000_0110],
        ];
        let permutation_key = [1, 2, 0];
        let output = super::permute_block_set(&blocks, &permutation_key);
        assert_eq!(output, [
            &[0b0000_0011, 0b0000_0100],
            &[0b0000_0101, 0b0000_0110],
            &[0b0000_0001, 0b0000_0010],
        ]);
    }

    #[test]
    fn test_depermute_block_set() {
        let blocks = [
            &[0b0000_0011, 0b0000_0100],
            &[0b0000_0101, 0b0000_0110],
            &[0b0000_0001, 0b0000_0010],
        ];
        let permutation_key = [1, 2, 0];
        let output = super::depermute_block_set(&blocks, &permutation_key);
        assert_eq!(output, [
            &[0b0000_0001, 0b0000_0010],
            &[0b0000_0011, 0b0000_0100],
            &[0b0000_0101, 0b0000_0110],
        ]);
    }
}