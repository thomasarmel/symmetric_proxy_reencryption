use collar::CollectArray;

pub(crate) fn xor_array<const ARRAY_SIZE: usize>(a : &[u8; ARRAY_SIZE], b: &[u8; ARRAY_SIZE]) -> [u8; ARRAY_SIZE] {
    let c =  a.iter()
        .zip(b.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect_array();
    c
}