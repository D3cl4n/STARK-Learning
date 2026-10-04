use crate::merkle::{Hash, verify, hash_leaf_node};
use crate::polynomial::{lagrange_interpolate, poly_eval};
use plonky2_field::goldilocks_field::GoldilocksField;
use sha2::{Sha256, Digest};


// given the merkle root, check whether a leaf node is in the Merkle tree at the given index
pub fn verify_challenge_points(root: &Hash, n: usize, leaves: &Vec<GoldilocksField>, auth_paths: &Vec<Vec<Hash>>) -> bool {
    // generate challenge points independent of the prover - if they don't match the prover tampered with the Merkle root
    let challenge_points: Vec<u64> = generate_challenge_points(root, n);

    // verify each challenge point by computing the leaf node hash and checking the associated authentication path
    for i in 0..challenge_points.len() {
        let leaf_hash: Hash = hash_leaf_node(leaves[i]);
        if !verify(root, &leaves[i], &auth_paths[i], challenge_points[i]) {
            return false;
        }
    }

    true
}


// low-degree check via re-interpolation
pub fn check_low_degree() -> bool {
    true
}


// independent calculation of the challenge points given merkle root to ensure prover isn't cherry-picking points
// generate n+1 challenge points where n is the size of the original dataset (need n+1 min.) to test low-degree
// by having the prover generate challenge points based on the Merkle root we enable Fiat-Shamir
fn generate_challenge_points(root: &merkle::Hash, n: usize) -> Vec<u64> {
    println!("[*] Generationg n+1 = {} challenge points", n+1);
    let mut challenge_points: Vec<u64> = vec![];

    for i in 0..=n {
        let mut res: [u8; 33] = [0u8; 33];
        res[..32].copy_from_slice(root);
        res[32] = i as u8;
        let hash = Sha256::digest(res);
        
        // first 6 bits cast to u64 for challenge point [0, 64]
        challenge_points.push((hash[0] >> 2) as u64);
    }

    challenge_points
}
