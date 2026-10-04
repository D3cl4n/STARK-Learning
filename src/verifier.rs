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
        if !verify(root, &leaf_hash, &auth_paths[i], challenge_points[i] as usize) {
            return false;
        }
    }

    true
}


// low-degree check via re-interpolation
pub fn check_low_degree(revealed_points: &[(GoldilocksField, GoldilocksField)], expected_degree: usize) -> bool {
    println!("[*] Recovering low-degree polynomial for the revealed points");

    // do Lagrange interpolation again which is inefficient and can be improved
    let needed_points: usize = expected_degree + 1;
    if revealed_points.len() < needed_points {
        return false; // to uniquely define a degree D polynomial we need D+1 points
    }

    // recover low-degree polynomial going through all but 1 of the revealed points
    let lagrange_coeffs: Vec<GoldilocksField> = lagrange_interpolate(&revealed_points[0..needed_points]);
    // evaluate the recovered polynomial at the last point to check consistency
    revealed_points[needed_points].1 == poly_eval(&lagrange_coeffs, revealed_points[needed_points].0)
}


// independent calculation of the challenge points given merkle root to ensure prover isn't cherry-picking points
// generate n+1 challenge points where n is the size of the original dataset (need n+1 min.) to test low-degree
// by having the prover generate challenge points based on the Merkle root we enable Fiat-Shamir
fn generate_challenge_points(root: &Hash, n: usize) -> Vec<u64> {
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
    println!("[*] Generated challenge points: {:?}", challenge_points);

    challenge_points
}
