use crate::polynomial;
use crate::merkle::{Hash, verify};


// given the merkle root, check whether a leaf node is in the Merkle tree at the given index
pub fn verify_challenge(root: &Hash, leaf: &Hash, idx: usize, auth_path: &[Hash]) -> bool {
    verify(root, leaf, auth_path, idx)
}


// generate challenge points (hardcoded for now)
pub fn generate_challenge_points() -> Vec<u64> {
    vec![5u64]
}