use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::PrimeField64;
use sha2::{Sha256, Digest};


// hash type for the Merkle tree
type Hash = [u8; 32];


// struct for the Merkle tree
pub struct MerkleTree {
    layers: Vec<Vec<Hash>> // the first inner vector corresponds to the 1st layer or the leaves; last layer is the root
}


// compute the hash of a leaf node
fn hash_leaf_node(value: GoldilocksField) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.to_canonical_u64().to_le_bytes());
    hasher.finalize().into() // .into() converts to [u8; 32]
}


// compute the hash of a node with 2 children
fn hash_parent_node(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}


// commit: computes the Merkle root given the vector of values
pub fn commit(values: &[GoldilocksField]) -> MerkleTree {
    // ensure the number of values is a multiple of 2
    assert_eq!(values.len() & values.len() - 1, 0);
    // create the first layer of leaves
    let mut layers: Vec<Vec<Hash>> = vec![values.iter().map(|&v| hash_leaf_node(v)).collect()];

    // loop until the root is ready to be calculated
    while layers.last().unwrap().len() > 1 {
        let prev_layer: &Vec<Hash> = layers.last().unwrap();
        let mut next_layer: Vec<Hash> = Vec::with_capacity(prev_layer.len() / 2); // upfront heap allocation since final length is known
        // loop over the pairs of elements in the previous layer
        for i in (0..prev_layer.len()).step_by(2) {
            let node: Hash = hash_parent_node(&prev_layer[i-1], &prev_layer[i]);
        }
    }

    MerkleTree {
        layers
    }
}