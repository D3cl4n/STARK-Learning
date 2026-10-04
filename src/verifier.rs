use crate::merkle::{Hash, verify};
use crate::polynomial::{lagrange_interpolate, poly_eval};


// given the merkle root, check whether a leaf node is in the Merkle tree at the given index
pub fn verify_challenge(root: &Hash, leaf: &Hash, idx: usize, auth_path: &[Hash]) -> bool {
    verify(root, leaf, auth_path, idx)
}


// generate n+1 challenge points where n is the size of the original dataset (need n+1) to test low-degree
pub fn generate_challenge_points(n: usize) -> Vec<u64> {
    vec![5u64]
}