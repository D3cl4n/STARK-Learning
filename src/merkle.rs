use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;
use sha2::{Sha256, Digest};


// hash type for the Merkle tree
pub type Hash = [u8; 32];


// compute the hash of a leaf node
pub fn hash_leaf_node(value: GoldilocksField) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.to_canonical_u64().to_le_bytes());
    hasher.finalize().into(); // TODO: document what .into() does here and how it converts into a Hash type
}


// compute the hash of a node with 2 children
pub fn hash_parent_node(left: )