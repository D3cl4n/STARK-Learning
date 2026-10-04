use crate::merkle::{Hash, verify};
use crate::polynomial::{lagrange_interpolate, poly_eval};


// given the merkle root, check whether a leaf node is in the Merkle tree at the given index
pub fn verify_challenge(root: &Hash, leaf: &Hash, idx: usize, auth_path: &[Hash]) -> bool {
    verify(root, leaf, auth_path, idx)
}
